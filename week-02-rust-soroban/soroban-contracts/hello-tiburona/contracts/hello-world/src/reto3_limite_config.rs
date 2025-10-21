#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracterror, contracttype,
    Env, Symbol, Address, String
};

// 🆕 RETO 3: Agregamos nuevo error LimiteInvalido
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NombreVacio = 1,
    NombreMuyLargo = 2,
    NoAutorizado = 3,
    NoInicializado = 4,
    LimiteInvalido = 5,  // 🆕 Para validar límites
}

// 🆕 RETO 3: Agregamos LimiteNombre a DataKey
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    ContadorSaludos,
    UltimoSaludo(Address),
    LimiteNombre,  // 🆕 Límite configurable de caracteres
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
        
        // 🆕 RETO 3: Inicializar límite por defecto en 32
        env.storage()
            .instance()
            .set(&DataKey::LimiteNombre, &32u32);
        
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
        
        // 🆕 RETO 3: Validar contra el límite configurable
        let limite: u32 = env.storage()
            .instance()
            .get(&DataKey::LimiteNombre)
            .unwrap_or(32);
        
        if nombre.len() > limite {
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
    
    // 🆕 RETO 3: Configurar el límite de caracteres (solo admin)
    /// Permite al admin cambiar el límite máximo de caracteres
    /// El límite debe estar entre 1 y 256 caracteres
    pub fn set_limite_nombre(
        env: Env,
        caller: Address,
        nuevo_limite: u32
    ) -> Result<(), Error> {
        // Verificar permisos
        let admin: Address = env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NoInicializado)?;
        
        if caller != admin {
            return Err(Error::NoAutorizado);
        }
        
        // Validar que el límite sea razonable (entre 1 y 256)
        if nuevo_limite == 0 || nuevo_limite > 256 {
            return Err(Error::LimiteInvalido);
        }
        
        // Guardar nuevo límite
        env.storage()
            .instance()
            .set(&DataKey::LimiteNombre, &nuevo_limite);
        
        env.storage()
            .instance()
            .extend_ttl(100, 100);
        
        Ok(())
    }
    
    // 🆕 RETO 3: Obtener el límite actual
    /// Retorna el límite máximo de caracteres configurado
    pub fn get_limite_nombre(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::LimiteNombre)
            .unwrap_or(32)
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
        assert_eq!(client.get_limite_nombre(), 32);  // 🆕 Límite por defecto
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
    
    // 🆕 RETO 3: Test de cambiar límite
    #[test]
    fn test_set_limite_nombre() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        // Cambiar límite a 10 caracteres
        client.set_limite_nombre(&admin, &10);
        assert_eq!(client.get_limite_nombre(), 10);
        
        // Nombre de 10 caracteres debe funcionar
        let nombre_ok = String::from_str(&env, "NombreLarg");  // 10 chars
        let resultado = client.hello(&usuario, &nombre_ok);
        assert_eq!(resultado, Symbol::new(&env, "Hola"));
    }
    
    // 🆕 RETO 3: Test de límite invalido (cero)
    #[test]
    #[should_panic(expected = "Error(Contract, #5)")]
    fn test_limite_invalido_cero() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        
        client.initialize(&admin);
        
        // Intentar poner límite en 0 (inválido)
        client.set_limite_nombre(&admin, &0);
    }
    
    // 🆕 RETO 3: Test de límite invalido (muy grande)
    #[test]
    #[should_panic(expected = "Error(Contract, #5)")]
    fn test_limite_invalido_muy_grande() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        
        client.initialize(&admin);
        
        // Intentar poner límite en 300 (inválido, máximo es 256)
        client.set_limite_nombre(&admin, &300);
    }
    
    // 🆕 RETO 3: Test de solo admin puede cambiar límite
    #[test]
    #[should_panic(expected = "Error(Contract, #3)")]
    fn test_solo_admin_cambia_limite() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        // Usuario no-admin intenta cambiar límite
        client.set_limite_nombre(&usuario, &20);
    }
    
    // 🆕 RETO 3: Test de que el límite se respeta
    #[test]
    #[should_panic(expected = "Error(Contract, #2)")]
    fn test_limite_se_respeta() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        // Cambiar límite a 5 caracteres
        client.set_limite_nombre(&admin, &5);
        
        // Intentar nombre de 6 caracteres (debe fallar)
        let nombre_largo = String::from_str(&env, "NombreL");  // 7 chars
        client.hello(&usuario, &nombre_largo);
    }
}