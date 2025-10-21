// HECHO POR LA TIBURONA SOFIA NAVARRO
// ============================================================================
// IMPLEMENTACIÓN DEL CONTRATO HELLO TIBURONA
// ============================================================================
// Este bloque implementa todas las funciones públicas del contrato.
// El macro #[contractimpl] convierte estas funciones en endpoints del contrato
// que pueden ser llamados desde fuera de la blockchain.

#[contractimpl]
impl HelloContract {
    
    // ========================================================================
    // FUNCIÓN: initialize
    // ========================================================================
    // Propósito: Configurar el contrato por primera vez
    // Parámetros:
    //   - env: Ambiente de Soroban (acceso a storage, eventos, etc.)
    //   - admin: Dirección que será el administrador del contrato
    // Retorna: Result<(), Error> - Ok si exitoso, Err si falla
    // ========================================================================
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        
        // PASO 1: Verificar que el contrato NO esté ya inicializado
        // --------------------------------------------------------
        // .has() verifica si existe una key en storage sin deserializar el valor
        // Es más eficiente que .get() porque solo chequea existencia
        if env.storage().instance().has(&DataKey::Admin) {
            // Si ya hay un admin guardado, el contrato ya fue inicializado
            // Retornamos error #4 (NoInicializado) para evitar re-inicialización
            return Err(Error::NoInicializado);
        }
        
        // PASO 2: Guardar el administrador en Instance Storage
        // -----------------------------------------------------
        // Instance storage = datos globales del contrato
        // .set() toma dos parámetros: key y valor (ambos por referencia &)
        env.storage()
            .instance()                          // Acceder a instance storage
            .set(&DataKey::Admin, &admin);       // Guardar admin con key "Admin"
        
        // PASO 3: Inicializar contador de saludos en 0
        // ---------------------------------------------
        // &0u32 = referencia a un unsigned 32-bit integer con valor 0
        // Inicializamos explícitamente en 0 para documentar el estado inicial
        env.storage()
            .instance()
            .set(&DataKey::ContadorSaludos, &0u32);
        
        // PASO 4: Extender el TTL (Time To Live) del instance storage
        // ------------------------------------------------------------
        // TTL = cuánto tiempo persisten los datos en blockchain
        // extend_ttl(100, 100) significa:
        //   - Si quedan menos de 100 ledgers de vida (threshold)
        //   - Extiende por 100 ledgers más (extend_to)
        // Esto asegura que la configuración no expire accidentalmente
        env.storage()
            .instance()
            .extend_ttl(100, 100);
        
        // PASO 5: Retornar éxito
        // -----------------------
        // Ok(()) = operación exitosa sin valor de retorno
        Ok(())
    }
    
    // ========================================================================
    // FUNCIÓN: hello
    // ========================================================================
    // Propósito: Saludar a una Tiburona y registrar el evento
    // Parámetros:
    //   - env: Ambiente de Soroban
    //   - usuario: Dirección de quien saluda
    //   - nombre: Nombre de la Tiburona (String para validar longitud)
    // Retorna: Result<Symbol, Error> - "Hola" si exitoso, Error si falla
    // ========================================================================
    pub fn hello(
        env: Env,
        usuario: Address,
        nombre: String
    ) -> Result<Symbol, Error> {
        
        // VALIDACIÓN 1: Nombre no puede estar vacío
        // ------------------------------------------
        // .len() retorna la longitud del String
        // Validamos PRIMERO porque es lo más barato (no requiere storage)
        if nombre.len() == 0 {
            // Early return: salir inmediatamente si hay error
            // Error #1 = NombreVacio
            return Err(Error::NombreVacio);
        }
        
        // VALIDACIÓN 2: Nombre no puede ser muy largo
        // --------------------------------------------
        // Límite de 32 caracteres para:
        //   - Prevenir consumo excesivo de gas
        //   - Evitar ataques de DoS con strings gigantes
        if nombre.len() > 32 {
            // Error #2 = NombreMuyLargo
            return Err(Error::NombreMuyLargo);
        }
        
        // PASO 1: Incrementar contador global de saludos
        // -----------------------------------------------
        // Patrón: Leer → Modificar → Guardar
        
        // 1a. Definir la key para el contador
        let key_contador = DataKey::ContadorSaludos;
        
        // 1b. Leer el contador actual del storage
        // .get() retorna Option<u32>
        // .unwrap_or(0) convierte None en 0 (lazy initialization)
        let contador: u32 = env.storage()
            .instance()
            .get(&key_contador)
            .unwrap_or(0);               // Primera vez = 0
        
        // 1c. Guardar el nuevo valor (contador + 1)
        // &(contador + 1) = referencia a la expresión calculada
        env.storage()
            .instance()
            .set(&key_contador, &(contador + 1));
        
        // PASO 2: Guardar el saludo específico del usuario
        // -------------------------------------------------
        // Persistent storage = datos críticos por usuario
        // .clone() crea una copia de usuario porque lo usamos dos veces
        env.storage()
            .persistent()                                      // Storage persistente
            .set(&DataKey::UltimoSaludo(usuario.clone()), &nombre);  // Key compuesta
        
        // PASO 3: Extender TTL del saludo del usuario
        // --------------------------------------------
        // Cada usuario tiene su propio TTL
        // Esto asegura que su saludo no expire mientras esté activo
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::UltimoSaludo(usuario), 100, 100);
        
        // PASO 4: Extender TTL del instance storage
        // ------------------------------------------
        // Mantener vivo el contador global también
        env.storage()
            .instance()
            .extend_ttl(100, 100);
        
        // PASO 5: Retornar saludo exitoso
        // --------------------------------
        // Symbol::new() crea un Symbol en el ambiente
        // Ok() envuelve el Symbol en Result para indicar éxito
        Ok(Symbol::new(&env, "Hola"))
    }
    
    // ========================================================================
    // FUNCIÓN: get_contador
    // ========================================================================
    // Propósito: Consultar el contador total de saludos
    // Retorna: u32 - Número total de saludos (nunca falla)
    // ========================================================================
    pub fn get_contador(env: Env) -> u32 {
        // Esta es una función de SOLO LECTURA
        // No modifica estado → más barata de ejecutar
        // Retorna u32 directamente (no Result) porque nunca falla
        
        env.storage()
            .instance()                          // Leer de instance storage
            .get(&DataKey::ContadorSaludos)      // Obtener contador
            .unwrap_or(0)                        // Si no existe, retornar 0
        
        // unwrap_or(0) es seguro aquí porque:
        // - No existe = 0 es un valor válido (nadie ha saludado)
        // - No hay posibilidad de panic
    }
    
    // ========================================================================
    // FUNCIÓN: get_ultimo_saludo
    // ========================================================================
    // Propósito: Consultar el último saludo de un usuario específico
    // Parámetros:
    //   - env: Ambiente de Soroban
    //   - usuario: Dirección del usuario a consultar
    // Retorna: Option<String> - Some(nombre) si existe, None si nunca saludó
    // ========================================================================
    pub fn get_ultimo_saludo(env: Env, usuario: Address) -> Option<String> {
        // Función de SOLO LECTURA
        // Retorna Option<String> porque el usuario puede no haber saludado nunca
        
        env.storage()
            .persistent()                         // Leer de persistent storage
            .get(&DataKey::UltimoSaludo(usuario)) // Key compuesta por usuario
        
        // .get() retorna Option<String> automáticamente
        // - Some(nombre) = el usuario ha saludado
        // - None = el usuario nunca ha saludado (válido)
        
        // NO usamos unwrap_or() porque queremos distinguir:
        //   - "No ha saludado" (None) vs "Saludó con texto vacío" (Some(""))
    }
    
    // ========================================================================
    // FUNCIÓN: reset_contador
    // ========================================================================
    // Propósito: Resetear el contador a 0 (SOLO ADMIN)
    // Parámetros:
    //   - env: Ambiente de Soroban
    //   - caller: Dirección de quien llama esta función
    // Retorna: Result<(), Error> - Ok si exitoso, Error si no autorizado
    // ========================================================================
    pub fn reset_contador(env: Env, caller: Address) -> Result<(), Error> {
        
        // PASO 1: Obtener el admin guardado en storage
        // ---------------------------------------------
        // Patrón de verificación de permisos:
        
        let admin: Address = env.storage()
            .instance()
            .get(&DataKey::Admin)              // Intentar leer admin
            .ok_or(Error::NoInicializado)?;    // Si None → Error #4
        
        // Desglose de .ok_or()?:
        //   - .ok_or() convierte Option → Result
        //     • Some(admin) → Ok(admin)
        //     • None → Err(NoInicializado)
        //   - El ? propaga el error automáticamente
        //     • Si es Err, la función retorna inmediatamente
        //     • Si es Ok, extrae el valor (admin)
        
        // PASO 2: Verificar que caller sea el admin
        // ------------------------------------------
        // Control de acceso: solo el admin puede resetear
        if caller != admin {
            // Early return si no está autorizado
            // Error #3 = NoAutorizado
            return Err(Error::NoAutorizado);
        }
        
        // PASO 3: Resetear el contador a 0
        // ---------------------------------
        // Si llegamos aquí, caller ES el admin
        // Seguro modificar el estado
        env.storage()
            .instance()
            .set(&DataKey::ContadorSaludos, &0u32);
        
        // PASO 4: Retornar éxito
        // -----------------------
        Ok(())
    }
}

// ============================================================================
// MÓDULO DE TESTS
// ============================================================================
// #[cfg(test)] = este código solo se compila en modo test
// Permite verificar que el contrato funciona correctamente antes de deploy
// ============================================================================

#[cfg(test)]
mod test {
    // Importar todo del módulo padre (el contrato)
    use super::*;
    // Importar utilidades de testing de Soroban
    use soroban_sdk::{Env, testutils::Address as TestAddress};

    // ========================================================================
    // TEST: Inicialización básica
    // ========================================================================
    // Verifica que el contrato se pueda inicializar correctamente
    // ========================================================================
    #[test]
    fn test_initialize() {
        // ARRANGE: Preparar el ambiente de test
        // --------------------------------------
        // Env::default() crea un ambiente simulado de blockchain
        let env = Env::default();
        
        // Registrar el contrato en el ambiente de test
        // .register() retorna el ID del contrato
        let contract_id = env.register(HelloContract, ());
        
        // Crear un cliente para interactuar con el contrato
        // El SDK genera HelloContractClient automáticamente
        let client = HelloContractClient::new(&env, &contract_id);
        
        // Generar una dirección aleatoria para admin
        let admin = Address::generate(&env);
        
        // ACT: Ejecutar la acción a testear
        // ----------------------------------
        client.initialize(&admin);
        
        // ASSERT: Verificar el resultado esperado
        // ----------------------------------------
        // El contador debe estar en 0 después de inicializar
        assert_eq!(client.get_contador(), 0);
    }
    
    // ========================================================================
    // TEST: No permitir re-inicialización
    // ========================================================================
    // Verifica que el contrato no se pueda inicializar dos veces
    // #[should_panic] = esperamos que este test cause panic
    // ========================================================================
    #[test]
    #[should_panic(expected = "Error(Contract, #4)")]  // Error #4 = NoInicializado
    fn test_no_reinicializar() {
        // ARRANGE
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        
        // ACT
        client.initialize(&admin);      // Primera inicialización - OK
        client.initialize(&admin);      // Segunda inicialización - DEBE FALLAR
        
        // ASSERT: El test PASA si hay panic con el mensaje esperado
    }
    
    // ========================================================================
    // TEST: Función hello exitosa
    // ========================================================================
    // Verifica que hello() funcione correctamente con inputs válidos
    // ========================================================================
    #[test]
    fn test_hello_exitoso() {
        // ARRANGE
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        // Generar dos direcciones: admin y usuario normal
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        // Inicializar el contrato primero
        client.initialize(&admin);
        
        // ACT
        // Crear un String en el ambiente de Soroban
        let nombre = String::from_str(&env, "Ana");
        let resultado = client.hello(&usuario, &nombre);
        
        // ASSERT: Verificar múltiples condiciones
        // ----------------------------------------
        // 1. La función retorna "Hola"
        assert_eq!(resultado, Symbol::new(&env, "Hola"));
        
        // 2. El contador se incrementó a 1
        assert_eq!(client.get_contador(), 1);
        
        // 3. El saludo del usuario se guardó correctamente
        assert_eq!(client.get_ultimo_saludo(&usuario), Some(nombre));
    }
    
    // ========================================================================
    // TEST: Validación de nombre vacío
    // ========================================================================
    // Verifica que hello() rechace nombres vacíos
    // ========================================================================
    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]  // Error #1 = NombreVacio
    fn test_nombre_vacio() {
        // ARRANGE
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        // ACT
        // Crear un String vacío
        let vacio = String::from_str(&env, "");
        client.hello(&usuario, &vacio);  // DEBE FALLAR con Error #1
        
        // ASSERT: El test PASA si hay panic con Error #1
    }
    
    // ========================================================================
    // TEST: Solo admin puede resetear
    // ========================================================================
    // Verifica que el admin pueda resetear el contador
    // ========================================================================
    #[test]
    fn test_reset_solo_admin() {
        // ARRANGE
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        // ACT
        // 1. Hacer un saludo (contador = 1)
        client.hello(&usuario, &String::from_str(&env, "Test"));
        assert_eq!(client.get_contador(), 1);
        
        // 2. Admin resetea el contador
        client.reset_contador(&admin);
        
        // ASSERT
        // El contador debe estar en 0 después del reset
        assert_eq!(client.get_contador(), 0);
    }
    
    // ========================================================================
    // TEST: No admin no puede resetear
    // ========================================================================
    // Verifica que usuarios NO admin no puedan resetear el contador
    // ========================================================================
    #[test]
    #[should_panic(expected = "Error(Contract, #3)")]  // Error #3 = NoAutorizado
    fn test_reset_no_autorizado() {
        // ARRANGE
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        // Generar dos direcciones diferentes
        let admin = Address::generate(&env);
        let otro = Address::generate(&env);   // Usuario NO admin
        
        client.initialize(&admin);
        
        // ACT
        // Un usuario NO admin intenta resetear
        client.reset_contador(&otro);  // DEBE FALLAR con Error #3
        
        // ASSERT: El test PASA si hay panic con Error #3
    }
}

// ============================================================================
// RESUMEN DE PATRONES APLICADOS
// ============================================================================
// 
// 1. VALIDACIONES:
//    - Orden: barato → caro (longitud antes de storage)
//    - Early returns para fail fast
//
// 2. STORAGE:
//    - Instance: datos globales (Admin, Contador)
//    - Persistent: datos por usuario (UltimoSaludo)
//    - Siempre extender TTL después de modificar
//
// 3. MANEJO DE ERRORES:
//    - Result<T, Error> para operaciones que pueden fallar
//    - Option<T> para valores que pueden no existir
//    - Operador ? para propagación automática de errores
//
// 4. CONTROL DE ACCESO:
//    - Verificar permisos ANTES de modificar estado
//    - Patrón: leer admin → comparar → ejecutar
//
// 5. TESTS:
//    - AAA: Arrange, Act, Assert
//    - should_panic para verificar que errores se manejen bien
//    - Testear casos exitosos Y casos de error
//
// ============================================================================