// src/lib.rs
#![no_std]

use soroban_sdk::{
    contract, contractimpl, contractevent,
    Address, Env, String
};

mod storage;
mod errors;

use storage::{DataKey, MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_DECIMALS};
use errors::TokenError;

// ============================================================================
// EVENTOS - Sistema nuevo con #[contractevent]
// ============================================================================

#[contractevent]
pub struct InitEvent {
    pub admin: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
}

#[contractevent]
pub struct MintEvent {
    pub admin: Address,
    pub to: Address,
    pub amount: i128,
}

#[contractevent]
pub struct BurnEvent {
    pub from: Address,
    pub amount: i128,
}

#[contractevent]
pub struct TransferEvent {
    pub from: Address,
    pub to: Address,
    pub amount: i128,
}

#[contractevent]
pub struct ApproveEvent {
    pub from: Address,
    pub spender: Address,
    pub amount: i128,
    pub live_until_ledger: u32,
}

// ============================================================================
// INTERFAZ DEL TOKEN
// ============================================================================

#[contract]
pub struct TokenBDB;

pub trait TokenTrait {
    /// Inicializa el token con metadatos y admin
    fn initialize(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
        decimals: u32,
    ) -> Result<(), TokenError>;

    /// Crea nuevos tokens (solo admin)
    fn mint(env: Env, to: Address, amount: i128) -> Result<(), TokenError>;

    /// Destruye tokens reduciendo supply
    fn burn(env: Env, from: Address, amount: i128) -> Result<(), TokenError>;

    /// Consulta balance de una cuenta
    fn balance(env: Env, id: Address) -> i128;

    /// Transfiere tokens entre cuentas
    fn transfer(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), TokenError>;

    /// Aprueba a otro usuario para gastar tokens
    fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        live_until_ledger: u32,
    ) -> Result<(), TokenError>;

    /// Consulta allowance entre dos cuentas
    fn allowance(env: Env, from: Address, spender: Address) -> i128;

    /// Transfiere tokens usando allowance
    fn transfer_from(
        env: Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), TokenError>;

    // Getters de metadata
    fn name(env: Env) -> String;
    fn symbol(env: Env) -> String;
    fn decimals(env: Env) -> u32;
    fn total_supply(env: Env) -> i128;
    fn admin(env: Env) -> Address;
}

// ============================================================================
// IMPLEMENTACIÓN
// ============================================================================

#[contractimpl]
impl TokenTrait for TokenBDB {
    fn initialize(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
        decimals: u32,
    ) -> Result<(), TokenError> {
        // 1. Verificar que no esté inicializado
        if env.storage().instance().has(&DataKey::Initialized) {
            return Err(TokenError::AlreadyInitialized);
        }

        // 2. Validar parámetros
        if name.len() == 0 || name.len() > MAX_NAME_LENGTH {
            return Err(TokenError::InvalidMetadata);
        }
        if symbol.len() == 0 || symbol.len() > MAX_SYMBOL_LENGTH {
            return Err(TokenError::InvalidMetadata);
        }
        if decimals > MAX_DECIMALS {
            return Err(TokenError::InvalidMetadata);
        }

        // 3. Requerir autenticación del admin
        admin.require_auth();

        // 4. Guardar configuración en instance storage
        env.storage().instance().set(&DataKey::Initialized, &true);
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TokenName, &name);
        env.storage().instance().set(&DataKey::TokenSymbol, &symbol);
        env.storage().instance().set(&DataKey::Decimals, &decimals);
        env.storage().instance().set(&DataKey::TotalSupply, &0i128);

        // 5. Emitir evento
        InitEvent {
            admin: admin.clone(),
            name: name.clone(),
            symbol: symbol.clone(),
            decimals,
        }.publish(&env);

        Ok(())
    }

    fn mint(env: Env, to: Address, amount: i128) -> Result<(), TokenError> {
        // 1. Validar amount
        if amount <= 0 {
            return Err(TokenError::InvalidAmount);
        }

        // 2. Verificar que solo el admin puede mintear
        let admin: Address = env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(TokenError::NotInitialized)?;

        admin.require_auth();

        // 3. Actualizar balance del destinatario
        let balance_key = DataKey::Balance(to.clone());
        let current_balance: i128 = env.storage()
            .persistent()
            .get(&balance_key)
            .unwrap_or(0);

        let new_balance = current_balance
            .checked_add(amount)
            .ok_or(TokenError::Overflow)?;

        env.storage().persistent().set(&balance_key, &new_balance);
        
        // 4. Extender TTL (60 días)
        env.storage()
            .persistent()
            .extend_ttl(&balance_key, 5184000, 6048000);

        // 5. Actualizar total supply
        let total_supply: i128 = env.storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);

        let new_total_supply = total_supply
            .checked_add(amount)
            .ok_or(TokenError::Overflow)?;

        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &new_total_supply);

        // 6. Emitir evento
        MintEvent {
            admin: admin.clone(),
            to: to.clone(),
            amount,
        }.publish(&env);

        Ok(())
    }

    fn burn(env: Env, from: Address, amount: i128) -> Result<(), TokenError> {
        // 1. Validar amount
        if amount <= 0 {
            return Err(TokenError::InvalidAmount);
        }

        // 2. Requerir autorización del dueño
        from.require_auth();

        // 3. Verificar balance suficiente
        let balance_key = DataKey::Balance(from.clone());
        let current_balance: i128 = env.storage()
            .persistent()
            .get(&balance_key)
            .unwrap_or(0);

        if current_balance < amount {
            return Err(TokenError::InsufficientBalance);
        }

        // 4. Actualizar balance
        let new_balance = current_balance - amount;

        if new_balance == 0 {
            // Optimización: eliminar key si balance = 0
            env.storage().persistent().remove(&balance_key);
        } else {
            env.storage().persistent().set(&balance_key, &new_balance);
            env.storage()
                .persistent()
                .extend_ttl(&balance_key, 5184000, 6048000);
        }

        // 5. Actualizar total supply
        let total_supply: i128 = env.storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);

        let new_total_supply = total_supply
            .checked_sub(amount)
            .ok_or(TokenError::Overflow)?;

        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &new_total_supply);

        // 6. Emitir evento
        BurnEvent {
            from: from.clone(),
            amount,
        }.publish(&env);

        Ok(())
    }

    fn balance(env: Env, id: Address) -> i128 {
        let balance_key = DataKey::Balance(id);
        env.storage().persistent().get(&balance_key).unwrap_or(0)
    }

    fn transfer(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), TokenError> {
        // 1. Validaciones básicas
        if amount <= 0 {
            return Err(TokenError::InvalidAmount);
        }

        if from == to {
            return Err(TokenError::SameAccount);
        }

        // 2. Requerir autorización
        from.require_auth();

        // 3. Verificar balance suficiente
        let from_key = DataKey::Balance(from.clone());
        let from_balance: i128 = env.storage()
            .persistent()
            .get(&from_key)
            .unwrap_or(0);

        if from_balance < amount {
            return Err(TokenError::InsufficientBalance);
        }

        // 4. Actualizar balance del sender
        let new_from_balance = from_balance - amount;
        if new_from_balance == 0 {
            env.storage().persistent().remove(&from_key);
        } else {
            env.storage().persistent().set(&from_key, &new_from_balance);
            env.storage()
                .persistent()
                .extend_ttl(&from_key, 5184000, 6048000);
        }

        // 5. Actualizar balance del receiver
        let to_key = DataKey::Balance(to.clone());
        let to_balance: i128 = env.storage()
            .persistent()
            .get(&to_key)
            .unwrap_or(0);

        let new_to_balance = to_balance
            .checked_add(amount)
            .ok_or(TokenError::Overflow)?;

        env.storage().persistent().set(&to_key, &new_to_balance);
        env.storage()
            .persistent()
            .extend_ttl(&to_key, 5184000, 6048000);

        // 6. Emitir evento
        TransferEvent {
            from: from.clone(),
            to: to.clone(),
            amount,
        }.publish(&env);

        Ok(())
    }

    fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        live_until_ledger: u32,
    ) -> Result<(), TokenError> {
        // 1. Validar amount (puede ser 0 para revocar)
        if amount < 0 {
            return Err(TokenError::InvalidAmount);
        }

        // 2. Requerir autorización
        from.require_auth();

        // 3. Actualizar allowance
        let allowance_key = DataKey::Allowance(from.clone(), spender.clone());

        if amount == 0 {
            // Revocar allowance
            env.storage().persistent().remove(&allowance_key);
        } else {
            env.storage().persistent().set(&allowance_key, &amount);
            env.storage()
                .persistent()
                .extend_ttl(&allowance_key, 5184000, 6048000);
        }

        // 4. Emitir evento
        ApproveEvent {
            from: from.clone(),
            spender: spender.clone(),
            amount,
            live_until_ledger,
        }.publish(&env);

        Ok(())
    }

    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        let allowance_key = DataKey::Allowance(from, spender);
        env.storage().persistent().get(&allowance_key).unwrap_or(0)
    }

    fn transfer_from(
        env: Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), TokenError> {
        // 1. Validaciones básicas
        if amount <= 0 {
            return Err(TokenError::InvalidAmount);
        }

        if from == to {
            return Err(TokenError::SameAccount);
        }

        // 2. Requerir autorización del spender
        spender.require_auth();

        // 3. Verificar allowance suficiente
        let allowance_key = DataKey::Allowance(from.clone(), spender.clone());
        let current_allowance: i128 = env.storage()
            .persistent()
            .get(&allowance_key)
            .unwrap_or(0);

        if current_allowance < amount {
            return Err(TokenError::InsufficientAllowance);
        }

        // 4. Verificar balance suficiente
        let from_key = DataKey::Balance(from.clone());
        let from_balance: i128 = env.storage()
            .persistent()
            .get(&from_key)
            .unwrap_or(0);

        if from_balance < amount {
            return Err(TokenError::InsufficientBalance);
        }

        // 5. Actualizar balance del sender
        let new_from_balance = from_balance - amount;
        if new_from_balance == 0 {
            env.storage().persistent().remove(&from_key);
        } else {
            env.storage().persistent().set(&from_key, &new_from_balance);
            env.storage()
                .persistent()
                .extend_ttl(&from_key, 5184000, 6048000);
        }

        // 6. Actualizar balance del receiver
        let to_key = DataKey::Balance(to.clone());
        let to_balance: i128 = env.storage()
            .persistent()
            .get(&to_key)
            .unwrap_or(0);

        let new_to_balance = to_balance
            .checked_add(amount)
            .ok_or(TokenError::Overflow)?;

        env.storage().persistent().set(&to_key, &new_to_balance);
        env.storage()
            .persistent()
            .extend_ttl(&to_key, 5184000, 6048000);

        // 7. Actualizar allowance
        let new_allowance = current_allowance - amount;
        if new_allowance == 0 {
            env.storage().persistent().remove(&allowance_key);
        } else {
            env.storage().persistent().set(&allowance_key, &new_allowance);
            env.storage()
                .persistent()
                .extend_ttl(&allowance_key, 5184000, 6048000);
        }

        // 8. Emitir evento
        TransferEvent {
            from: from.clone(),
            to: to.clone(),
            amount,
        }.publish(&env);

        Ok(())
    }

    // ========================================================================
    // GETTERS
    // ========================================================================

    fn name(env: Env) -> String {
        if !env.storage().instance().has(&DataKey::Initialized) {
            return String::from_str(&env, "");
        }

        env.storage()
            .instance()
            .get(&DataKey::TokenName)
            .unwrap_or(String::from_str(&env, ""))
    }

    fn symbol(env: Env) -> String {
        if !env.storage().instance().has(&DataKey::Initialized) {
            return String::from_str(&env, "");
        }

        env.storage()
            .instance()
            .get(&DataKey::TokenSymbol)
            .unwrap_or(String::from_str(&env, ""))
    }

    fn decimals(env: Env) -> u32 {
        if !env.storage().instance().has(&DataKey::Initialized) {
            return 0;
        }

        env.storage().instance().get(&DataKey::Decimals).unwrap_or(0)
    }

    fn total_supply(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }

    fn admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("Admin not initialized")
    }
}

// ============================================================================
// MÓDULO DE TESTS
// ============================================================================

#[cfg(test)]
mod test;