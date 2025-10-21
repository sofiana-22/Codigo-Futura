#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracterror, contracttype,
    Env, Symbol, Address, String
};

// Definición de errores personalizados
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NombreVacio = 1,
    NombreMuyLargo = 2,
    NoAutorizado = 3,
    NoInicializado = 4,
}

// 🆕 RETO 1: Agregamos ContadorPorUsuario(Address)
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    ContadorSaludos,
    UltimoSaludo(Address),
    ContadorPorUsuario(Address),  // 🆕 Contador individual por usuario
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
        
        // Validaciones
        if nombre.len() == 0 {
            return Err(Error::NombreVacio);
        }
        
        if nombre.len() > 32 {
            return Err(Error::NombreMuyLargo);
        }
        
        // Incrementar contador global
        let key_contador = DataKey::ContadorSaludos;
        let contador: u32 = env.storage()
            .instance()
            .get(&key_contador)
            .unwrap_or(0);
        
        env.storage()
            .instance()
            .set(&key_contador, &(contador + 1));
        
        // Guardar último saludo
        env.storage()
            .persistent()
            .set(&DataKey::UltimoSaludo(usuario.clone()), &nombre);
        
        // 🆕 RETO 1: Incrementar contador individual del usuario
        let contador_usuario: u32 = env.storage()
            .persistent()
            .get(&DataKey::ContadorPorUsuario(usuario.clone()))
            .unwrap_or(0);
        
        env.storage()
            .persistent()
            .set(&DataKey::ContadorPorUsuario(usuario.clone()), &(contador_usuario + 1));
        
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::ContadorPorUsuario(usuario.clone()), 100, 100);
        
        // Extender TTL
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
    
    // 🆕 RETO 1: Nueva función para obtener contador individual
    pub fn get_contador_usuario(env: Env, usuario: Address) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::ContadorPorUsuario(usuario))
            .unwrap_or(0)
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
    
    // 🆕 RETO 1: Test de contador individual por usuario
    #[test]
    fn test_contador_por_usuario() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let maria = Address::generate(&env);
        let ana = Address::generate(&env);
        
        client.initialize(&admin);
        
        // María saluda 3 veces
        client.hello(&maria, &String::from_str(&env, "Maria1"));
        assert_eq!(client.get_contador_usuario(&maria), 1);
        
        client.hello(&maria, &String::from_str(&env, "Maria2"));
        assert_eq!(client.get_contador_usuario(&maria), 2);
        
        client.hello(&maria, &String::from_str(&env, "Maria3"));
        assert_eq!(client.get_contador_usuario(&maria), 3);
        
        // Ana saluda 1 vez
        client.hello(&ana, &String::from_str(&env, "Ana1"));
        assert_eq!(client.get_contador_usuario(&ana), 1);
        
        // Verificar contadores independientes
        assert_eq!(client.get_contador_usuario(&maria), 3);  // María: 3
        assert_eq!(client.get_contador_usuario(&ana), 1);    // Ana: 1
        assert_eq!(client.get_contador(), 4);                 // Total: 4
    }
}