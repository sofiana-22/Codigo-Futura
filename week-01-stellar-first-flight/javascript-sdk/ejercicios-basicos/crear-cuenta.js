/**
 * CREATE STELLAR ACCOUNT - BASIC VERSION
 * 
 * This script demonstrates how to:
 * 1. Generate a cryptographically secure keypair
 * 2. Fund the account using Friendbot (testnet only)
 * 3. Display the public and secret keys
 * 
 * IMPORTANT: Never share or commit your secret key!
 * 
 * Usage: node ejercicios-basicos/crear-cuenta.js
 */

const StellarSdk = require('@stellar/stellar-sdk');
const fetch = require('node-fetch');

/**
 * Creates a new Stellar account and funds it with Friendbot
 * @returns {Promise<StellarSdk.Keypair>} The generated keypair
 */
async function crearCuenta() {
  console.log('🔐 Generating your new keypair...\n');
  
  // Generate a random keypair (public + secret key)
  // This uses cryptographically secure random number generation
  const pair = StellarSdk.Keypair.random();
  
  console.log('✅ Account created!\n');
  
  // Public key - Safe to share, used to receive payments
  console.log('📧 PUBLIC KEY (you can share this):');
  console.log(pair.publicKey());
  
  // Secret key - NEVER share this! It controls your money
  console.log('\n🔑 SECRET KEY (NEVER SHARE):');
  console.log(pair.secret());
  
  console.log('\n💰 Funding with Friendbot...');
  
  try {
    // Friendbot is a testnet-only service that gives free XLM
    // On mainnet, you need to buy XLM from an exchange
    const response = await fetch(
      `https://friendbot.stellar.org/?addr=${pair.publicKey()}`
    );
    
    const result = await response.json();
    
    // Check if funding was successful
    if (result.successful || response.ok) {
      console.log('✅ Account funded with 10,000 XLM!\n');
      console.log('🔗 Transaction hash:', result.hash);
    }
  } catch (error) {
    console.error('❌ Error funding account:', error.message);
  }
  
  console.log('\n⚠️  IMPORTANT: Save these keys in a secure place\n');
  console.log('💡 TIP: Copy them to a .env file (see .env.example)\n');
  
  return pair;
}

// Execute the function
crearCuenta();