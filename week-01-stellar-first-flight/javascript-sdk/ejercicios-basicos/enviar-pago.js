/**
 * SEND PAYMENT - BASIC VERSION
 * 
 * This script demonstrates how to:
 * 1. Load a Stellar account
 * 2. Build a payment transaction
 * 3. Sign the transaction with your secret key
 * 4. Submit it to the network
 * 
 * SECURITY: Replace the hardcoded SECRET_KEY with environment variables
 * in production! See .env.example
 * 
 * Usage: node ejercicios-basicos/enviar-pago.js
 */

const StellarSdk = require('@stellar/stellar-sdk');

// Initialize Horizon server (testnet)
const server = new StellarSdk.Horizon.Server('https://horizon-testnet.stellar.org');

// Network passphrase identifies which Stellar network we're using
const networkPassphrase = StellarSdk.Networks.TESTNET;

// ⚠️ SECURITY WARNING: Never commit real secret keys to git!
// Use environment variables instead (see .env.example)
const SECRET_KEY = 'SALHUQSCUP37RMYM67R4I7J5LVFOX63F4CS6YQ2UHVQV54W7JWH5I745';
const DESTINATION = 'GDBURBUQHHVDKFYP53446SWBQ2Q3ULA4OKDFDWQPCMUPXTZ7PZWRFOLY';

/**
 * Sends a payment transaction on Stellar network
 * @param {string} amount - Amount of XLM to send
 * @param {string} memo - Optional memo text for the transaction
 * @returns {Promise<object>} Transaction result from the network
 */
async function enviarPago(amount, memo = '') {
  try {
    // STEP 1: Load the source account from the network
    // We need the current sequence number to build the transaction
    const sourceKeys = StellarSdk.Keypair.fromSecret(SECRET_KEY);
    const sourceAccount = await server.loadAccount(sourceKeys.publicKey());
    
    console.log(`💰 Current balance: ${sourceAccount.balances[0].balance} XLM\n`);
    
    // STEP 2: Build the transaction
    // TransactionBuilder helps construct valid Stellar transactions
    const transaction = new StellarSdk.TransactionBuilder(sourceAccount, {
      fee: StellarSdk.BASE_FEE, // Fee per operation (0.00001 XLM)
      networkPassphrase: networkPassphrase // Ensures tx only valid on testnet
    })
      // Add a payment operation to the transaction
      .addOperation(StellarSdk.Operation.payment({
        destination: DESTINATION, // Who receives the payment
        asset: StellarSdk.Asset.native(), // Native asset = XLM
        amount: amount.toString() // Amount must be a string
      }))
      // Add memo (optional message) - useful for identifying payments
      .addMemo(memo ? StellarSdk.Memo.text(memo) : StellarSdk.Memo.none())
      // Set timeout (transaction invalid after 30 seconds)
      .setTimeout(30)
      .build();
    
    // STEP 3: Sign the transaction with your secret key
    // This proves you authorize this transaction
    transaction.sign(sourceKeys);
    
    // STEP 4: Submit to the network
    // The network will validate and include it in the next ledger
    const result = await server.submitTransaction(transaction);
    
    console.log('🎉 PAYMENT SUCCESSFUL!\n');
    console.log(`💰 You sent: ${amount} XLM`);
    console.log(`📝 Memo: ${memo || '(none)'}`);
    console.log(`🔗 Transaction Hash: ${result.hash}`);
    console.log(`🔍 View on: https://stellar.expert/explorer/testnet/tx/${result.hash}\n`);
    
    return result;
    
  } catch (error) {
    console.error('❌ ERROR:', error.message);
    
    // Show detailed error information if available
    if (error.response && error.response.data) {
      console.error('📋 Details:', JSON.stringify(error.response.data, null, 2));
    }
    
    throw error;
  }
}

// Execute: Send 25 XLM with a custom memo
enviarPago('25', '¡Mi primer pago con código! 🚀');