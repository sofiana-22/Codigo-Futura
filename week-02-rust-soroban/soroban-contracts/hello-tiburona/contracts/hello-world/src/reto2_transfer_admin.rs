#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracterror, contracttype,
    Env, Symbol, Address, String
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NombreVacio = 1,
    NombreMuyLargo = 2,
    NoAutorizado = 3,
    NoInicializado = 4,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    ContadorSaludos,
    UltimoSaludo(Address),
}

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::NoInicializado);
        }
        
        env.storage()
            .instance()
            .set(&DataKey::Admin, &admin);
        
        env.storage()
            .instance()
            .set(&DataKey::ContadorSaludos, &0u32);
        
        env.storage()
            .instance()
            .extend_ttl(100, 100);
        
        Ok(())
    }
    
    pub fn hello(
        env: Env,
        usuario: Address,
        nombre: String
    ) -> Result<Symbol, Error> {
        
        if nombre.len() == 0 {
            return Err(Error::NombreVacio);
        }
        
        if nombre.len() > 32 {
            return Err(Error::NombreMuyLargo);
        }
        
        let key_contador = DataKey::ContadorSaludos;
        let contador: u32 = env.storage()
            .instance()
            .get(&key_contador)
            .unwrap_or(0);
        
        env.storage()
            .instance()
            .set(&key_contador, &(contador + 1));
        
        env.storage()
            .persistent()
            .set(&DataKey::UltimoSaludo(usuario.clone()), &nombre);
        
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::UltimoSaludo(usuario), 100, 100);
        
        env.storage()
            .instance()
            .extend_ttl(100, 100);
        
        Ok(Symbol::new(&env, "Hola"))
    }
    
    pub fn get_contador(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::ContadorSaludos)
            .unwrap_or(0)
    }
    
    pub fn get_ultimo_saludo(env: Env, usuario: Address) -> Option<String> {
        env.storage()
            .persistent()
            .get(&DataKey::UltimoSaludo(usuario))
    }
    
    pub fn reset_contador(env: Env, caller: Address) -> Result<(), Error> {
        let admin: Address = env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NoInicializado)?;

        if caller != admin {
            return Err(Error::NoAutorizado);
        }

        env.storage()
            .instance()
            .set(&DataKey::ContadorSaludos, &0u32);
        
        Ok(())
    }
    
    // 🆕 RETO 2: Transferir ownership del contrato a un nuevo admin
    /// Permite al admin actual transferir el control a otra dirección
    pub fn transfer_admin(
        env: Env,
        current_admin: Address,
        new_admin: Address
    ) -> Result<(), Error> {
        // Verificar que el contrato está inicializado
        let admin: Address = env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NoInicializado)?;
        
        // Solo el admin actual puede transferir
        if current_admin != admin {
            return Err(Error::NoAutorizado);
        }
        
        // Guardar el nuevo admin
        env.storage()
            .instance()
            .set(&DataKey::Admin, &new_admin);
        
        // Extender TTL
        env.storage()
            .instance()
            .extend_ttl(100, 100);
        
        Ok(())
    }
    
    // 🆕 RETO 2: Obtener el admin actual
    /// Retorna la dirección del admin actual (útil para verificar)
    pub fn get_admin(env: Env) -> Option<Address> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, testutils::Address as TestAddress};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        
        client.initialize(&admin);
        
        assert_eq!(client.get_contador(), 0);
    }
    
    #[test]
    #[should_panic(expected = "Error(Contract, #4)")]
    fn test_no_reinicializar() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        
        client.initialize(&admin);
        client.initialize(&admin);
    }
    
    #[test]
    fn test_hello_exitoso() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        let nombre = String::from_str(&env, "Ana");
        let resultado = client.hello(&usuario, &nombre);
        
        assert_eq!(resultado, Symbol::new(&env, "Hola"));
        assert_eq!(client.get_contador(), 1);
        assert_eq!(client.get_ultimo_saludo(&usuario), Some(nombre));
    }
    
    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn test_nombre_vacio() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        let vacio = String::from_str(&env, "");
        client.hello(&usuario, &vacio);
    }
    
    #[test]
    fn test_reset_solo_admin() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        client.hello(&usuario, &String::from_str(&env, "Test"));
        assert_eq!(client.get_contador(), 1);
        
        client.reset_contador(&admin);
        assert_eq!(client.get_contador(), 0);
    }
    
    #[test]
    #[should_panic(expected = "Error(Contract, #3)")]
    fn test_reset_no_autorizado() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let otro = Address::generate(&env);
        
        client.initialize(&admin);
        
        client.reset_contador(&otro);
    }
    
    // 🆕 RETO 2: Test de transferencia exitosa
    #[test]
    fn test_transfer_admin_exitoso() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin1 = Address::generate(&env);
        let admin2 = Address::generate(&env);
        
        client.initialize(&admin1);
        
        // Verificar admin inicial
        assert_eq!(client.get_admin(), Some(admin1.clone()));
        
        // Transferir a nuevo admin
        client.transfer_admin(&admin1, &admin2);
        
        // Verificar nuevo admin
        assert_eq!(client.get_admin(), Some(admin2.clone()));
        
        // El nuevo admin puede resetear
        client.hello(&admin2, &String::from_str(&env, "Test"));
        client.reset_contador(&admin2);
        assert_eq!(client.get_contador(), 0);
    }
    
    // 🆕 RETO 2: Test de transferencia no autorizada
    #[test]
    #[should_panic(expected = "Error(Contract, #3)")]
    fn test_transfer_admin_no_autorizado() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let impostor = Address::generate(&env);
        let nuevo = Address::generate(&env);
        
        client.initialize(&admin);
        
        // Un no-admin intenta transferir
        client.transfer_admin(&impostor, &nuevo);
    }
    
    // 🆕 RETO 2: Test de que el admin viejo ya no tiene permisos
    #[test]
    #[should_panic(expected = "Error(Contract, #3)")]
    fn test_admin_viejo_pierde_permisos() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin1 = Address::generate(&env);
        let admin2 = Address::generate(&env);
        
        client.initialize(&admin1);
        
        // Transferir ownership
        client.transfer_admin(&admin1, &admin2);
        
        // El admin viejo NO puede resetear
        client.reset_contador(&admin1);
    }
}