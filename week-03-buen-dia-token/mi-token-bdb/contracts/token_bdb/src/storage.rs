// src/storage.rs
use soroban_sdk::{contracttype, Address};

/// Límites de validación
pub const MAX_NAME_LENGTH: u32 = 100;
pub const MAX_SYMBOL_LENGTH: u32 = 32;
pub const MAX_DECIMALS: u32 = 18;

/// Claves de almacenamiento del contrato
/// 
/// Separamos en dos tipos:
/// - Instance Storage: Metadata global (Admin, Name, Symbol, etc.)
/// - Persistent Storage: Datos de usuarios (Balance, Allowance)
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Balance de un usuario específico - Persistent
    Balance(Address),
    
    /// Allowance entre dos cuentas (owner, spender) - Persistent
    Allowance(Address, Address),
    
    /// Supply total de tokens - Instance
    TotalSupply,
    
    /// Dirección del administrador - Instance
    Admin,
    
    /// Nombre del token - Instance
    TokenName,
    
    /// Símbolo del token - Instance
    TokenSymbol,
    
    /// Número de decimales - Instance
    Decimals,
    
    /// Flag de inicialización - Instance
    Initialized,
}