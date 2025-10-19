/**
 * MASS ACCOUNT CREATION - ADVANCED VERSION
 * 
 * This script demonstrates how to:
 * 1. Create multiple Stellar accounts in batch
 * 2. Store account information in memory
 * 3. Display a summary of all created accounts
 * 
 * Use case: Creating multiple accounts for testing or airdrops
 * 
 * Usage: node ejercicios-avanzados/crear-cuenta-masivo.js
 */

const StellarSdk = require('@stellar/stellar-sdk');
const fetch = require('node-fetch');

// Array to store all created accounts
const cuentas = [];

/**
 * Creates a single Stellar account with a specific number identifier
 * @param {number} numero - Account number for identification
 * @returns {Promise<StellarSdk.Keypair>} The generated keypair
 */
async function crearCuenta(numero) {
  console.log(`🔐 Generating account ${numero}...\n`);
  
  // Generate a cryptographically secure random keypair
  const pair = StellarSdk.Keypair.random();
  
  console.log('✅ Account created!\n');
  console.log('📧 PUBLIC KEY (shareable):');
  console.log(pair.publicKey());
  console.log('\n🔑 SECRET KEY (NEVER SHARE):');
  console.log(pair.secret());
  
  console.log('\n💰 Funding with Friendbot...');
  
  try {
    // Request testnet XLM from Friendbot
    const response = await fetch(
      `https://friendbot.stellar.org/?addr=${pair.publicKey()}`
    );
    
    const result = await response.json();
    
    // Check if funding was successful
    if (result.successful || response.ok) {
      console.log('✅ Account funded with 10,000 XLM!\n');
      console.log('🔗 Transaction hash:', result.hash);
      
      // Store account information in the array
      cuentas.push({
        numero: numero,
        publicKey: pair.publicKey(),
        secretKey: pair.secret(),
        balance: '10000.0000000'
      });
    }
  } catch (error) {
    console.error('❌ Error funding account:', error.message);
  }
  
  console.log('\n⚠️  IMPORTANT: Save these keys in a secure place\n');
  
  return pair;
}

/**
 * Main function to create multiple accounts in batch
 * Creates 5 accounts and displays a final summary
 */
async function crearCuentasMasivas() {
  // Display header
  console.log('╔═══════════════════════════════════════════╗');
  console.log('  🚀 MASS STELLAR ACCOUNT CREATION');
  console.log('╚═══════════════════════════════════════════╝\n');
  
  // Create 5 accounts sequentially
  // Note: In production, consider rate limiting to avoid overwhelming Friendbot
  for (let i = 1; i <= 5; i++) {
    await crearCuenta(i);
    console.log('─────────────────────────────────────────\n');
  }
  
  // Display final summary of all created accounts
  console.log('╔═══════════════════════════════════════════╗');
  console.log('  📊 CREATED ACCOUNTS SUMMARY');
  console.log('╚═══════════════════════════════════════════╝\n');
  
  // Loop through all stored accounts
  cuentas.forEach((cuenta) => {
    console.log(`Account ${cuenta.numero}:`);
    console.log(`  Public Key:  ${cuenta.publicKey}`);
    console.log(`  Secret Key:  ${cuenta.secretKey}`);
    console.log(`  Balance:     ${cuenta.balance} XLM\n`);
  });
  
  console.log(`✅ Total accounts created: ${cuentas.length}`);
  console.log(`💰 Total XLM distributed: ${cuentas.length * 10000} XLM\n`);
  
  // Security reminder
  console.log('⚠️  SECURITY REMINDER:');
  console.log('   - Save all keys in a secure location');
  console.log('   - Consider using a password manager');
  console.log('   - NEVER commit keys to git\n');
}

// Execute mass account creation
crearCuentasMasivas();