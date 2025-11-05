// src/errors.rs
use soroban_sdk::contracterror;

/// Errores personalizados del token
/// 
/// Cada error tiene un código único que se verá en los logs
/// de Stellar cuando una transacción falle
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum TokenError {
    /// El contrato ya fue inicializado
    AlreadyInitialized = 1,
    
    /// Amount debe ser > 0 (o >= 0 para approve)
    InvalidAmount = 2,
    
    /// Balance insuficiente para la operación
    InsufficientBalance = 3,
    
    /// Allowance insuficiente para transfer_from
    InsufficientAllowance = 4,
    
    /// El contrato no ha sido inicializado
    NotInitialized = 5,
    
    /// Metadata inválido (name, symbol, decimals)
    InvalidMetadata = 6,
    
    /// Overflow en operación aritmética
    Overflow = 7,
    
    /// Transferencia a la misma cuenta (from == to)
    SameAccount = 8,
}