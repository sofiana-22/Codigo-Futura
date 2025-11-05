// src/test.rs
#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as TestAddress,
    Address, Env, String,
};

// ============================================================================
// TESTS DE INICIALIZACIÓN
// ============================================================================

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();  // ✅ AGREGADO
    
    let contract_id = env.register(TokenBDB, ());  // ✅ CORREGIDO
    let client = TokenBDBClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let name = String::from_str(&env, "Builder Token");
    let symbol = String::from_str(&env, "BDB");

    client.initialize(&admin, &name, &symbol, &7);

    assert_eq!(client.name(), name);
    assert_eq!(client.symbol(), symbol);
    assert_eq!(client.decimals(), 7);
    assert_eq!(client.total_supply(), 0);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]  // ✅ CORREGIDO
fn test_initialize_twice_fails() {
    let env = Env::default();
    env.mock_all_auths();  // ✅ AGREGADO
    
    let contract_id = env.register(TokenBDB, ());  // ✅ CORREGIDO
    let client = TokenBDBClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let name = String::from_str(&env, "Token");
    let symbol = String::from_str(&env, "TOK");

    client.initialize(&admin, &name, &symbol, &7);
    client.initialize(&admin, &name, &symbol, &7); // Debe fallar
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]  // ✅ CORREGIDO
fn test_invalid_decimals() {
    let env = Env::default();
    env.mock_all_auths();  // ✅ AGREGADO
    
    let contract_id = env.register(TokenBDB, ());  // ✅ CORREGIDO
    let client = TokenBDBClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    // Decimales > 18 debe fallar
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &19  // ❌ Inválido
    );
}

// ============================================================================
// TESTS DE MINT
// ============================================================================

#[test]
fn test_mint_and_balance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(TokenBDB, ());  // ✅ CORREGIDO
    let client = TokenBDBClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    client.initialize(
        &admin,
        &String::from_str(&env, "Builder Token"),
        &String::from_str(&env, "BDB"),
        &7
    );

    client.mint(&user, &1000);

    assert_eq!(client.balance(&user), 1000);
    assert_eq!(client.total_supply(), 1000);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]  // ✅ CORREGIDO
fn test_mint_zero_fails() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(TokenBDB, ());  // ✅ CORREGIDO
    let client = TokenBDBClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );

    client.mint(&user, &0); // Debe fallar
}

// ============================================================================
// TESTS DE TRANSFER
// ============================================================================

#[test]
fn test_transfer() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(TokenBDB, ());  // ✅ CORREGIDO
    let client = TokenBDBClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    client.initialize(
        &admin,
        &String::from_str(&env, "Builder Token"),
        &String::from_str(&env, "BDB"),
        &7
    );

    client.mint(&alice, &1000);
    client.transfer(&alice, &bob, &250);

    assert_eq!(client.balance(&alice), 750);
    assert_eq!(client.balance(&bob), 250);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]  // ✅ CORREGIDO
fn test_transfer_insufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(TokenBDB, ());  // ✅ CORREGIDO
    let client = TokenBDBClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );

    client.mint(&alice, &100);
    client.transfer(&alice, &bob, &200); // Debe fallar
}

#[test]
#[should_panic(expected = "Error(Contract, #8)")]  // ✅ CORREGIDO
fn test_transfer_to_self() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(TokenBDB, ());  // ✅ CORREGIDO
    let client = TokenBDBClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let alice = Address::generate(&env);

    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );

    client.mint(&alice, &1000);
    client.transfer(&alice, &alice, &100); // Debe fallar
}

// ============================================================================
// TESTS DE APPROVE Y TRANSFER_FROM
// ============================================================================

#[test]
fn test_approve_and_transfer_from() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(TokenBDB, ());  // ✅ CORREGIDO
    let client = TokenBDBClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let charlie = Address::generate(&env);

    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );

    client.mint(&alice, &1000);
    client.approve(&alice, &bob, &300, &1000000);

    assert_eq!(client.allowance(&alice, &bob), 300);

    client.transfer_from(&bob, &alice, &charlie, &200);

    assert_eq!(client.balance(&alice), 800);
    assert_eq!(client.balance(&charlie), 200);
    assert_eq!(client.allowance(&alice, &bob), 100);
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]  // ✅ CORREGIDO
fn test_transfer_from_insufficient_allowance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(TokenBDB, ());  // ✅ CORREGIDO
    let client = TokenBDBClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let charlie = Address::generate(&env);

    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );

    client.mint(&alice, &1000);
    client.approve(&alice, &bob, &100, &1000000);  // Solo 100 aprobados

    // Bob intenta transferir más de lo aprobado
    client.transfer_from(&bob, &alice, &charlie, &200); // Debe fallar
}

// ============================================================================
// TESTS DE BURN
// ============================================================================

#[test]
fn test_burn() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(TokenBDB, ());  // ✅ CORREGIDO
    let client = TokenBDBClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let alice = Address::generate(&env);

    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );

    client.mint(&alice, &1000);
    client.burn(&alice, &300);

    assert_eq!(client.balance(&alice), 700);
    assert_eq!(client.total_supply(), 700);
}

// ============================================================================
// TESTS SIN INICIALIZAR
// ============================================================================

#[test]
#[should_panic(expected = "Error(Contract, #5)")]  // ✅ CORREGIDO
fn test_operations_without_init() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(TokenBDB, ());  // ✅ CORREGIDO
    let client = TokenBDBClient::new(&env, &contract_id);

    let alice = Address::generate(&env);

    // Intentar mint sin inicializar debe fallar
    client.mint(&alice, &100);
}