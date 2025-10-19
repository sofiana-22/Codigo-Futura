/**
 * MULTIPLE PAYMENTS - ADVANCED VERSION
 * 
 * This script demonstrates how to:
 * 1. Send payments to multiple recipients in batch
 * 2. Handle errors gracefully (continue even if one fails)
 * 3. Track success/failure for each transaction
 * 4. Display a comprehensive summary
 * 
 * Use case: Payroll, airdrops, batch distributions
 * 
 * ⚠️ SECURITY: Replace SECRET_KEY with environment variable!
 * 
 * Usage: node ejercicios-avanzados/enviar-pago-multiple.js
 */

const StellarSdk = require('@stellar/stellar-sdk');

// Initialize Horizon server (testnet)
const server = new StellarSdk.Horizon.Server('https://horizon-testnet.stellar.org');
const networkPassphrase = StellarSdk.Networks.TESTNET;

// ⚠️ WARNING: Never commit real secret keys!
// Use environment variables: process.env.SECRET_KEY
const SECRET_KEY = 'SECRET KEY'; // Replace with your testnet secret key

// Array of recipients with their addresses and memo identifiers
const destinatarios = [
  { 
    publicKey: "GCDIJ4WO6FF6XCZ5RDI5XGCVC6TFZA63EFLY5O6I4USJNFQLJ4OYVIC3", 
    memo: "Payment-001" 
  },
  { 
    publicKey: "GCQIVKWPLYGR3EK2JHUAJZIS4HMTO7FGDHQMUUE5SBHTWXXZQF7MZ7RK", 
    memo: "Payment-002" 
  },
  { 
    publicKey: "GDCV4NX7DT6YANGJPB3LAXMYRLKU4ZVXVQKVYZM2IRFX6M4W6KKB3K7E", 
    memo: "Payment-003" 
  }
];

/**
 * Sends a payment to a single destination
 * @param {string} destinatario - Recipient's public key
 * @param {string} amount - Amount to send
 * @param {string} memo - Transaction memo for identification
 * @returns {Promise<object>} Transaction result
 */
async function enviarPago(destinatario, amount, memo) {
  try {
    console.log(`\n💸 Processing ${memo}...`);
    console.log(`📧 Recipient: ${destinatario.substring(0, 8)}...`);
    
    // STEP 1: Load source account
    const sourceKeys = StellarSdk.Keypair.fromSecret(SECRET_KEY);
    const sourceAccount = await server.loadAccount(sourceKeys.publicKey());
    
    console.log(`💰 Current balance: ${sourceAccount.balances[0].balance} XLM\n`);
    
    // STEP 2: Build transaction
    const transaction = new StellarSdk.TransactionBuilder(sourceAccount, {
      fee: StellarSdk.BASE_FEE,
      networkPassphrase: networkPassphrase
    })
      .addOperation(StellarSdk.Operation.payment({
        destination: destinatario,
        asset: StellarSdk.Asset.native(),
        amount: amount.toString()
      }))
      .addMemo(memo ? StellarSdk.Memo.text(memo) : StellarSdk.Memo.none())
      .setTimeout(30)
      .build();
    
    // STEP 3: Sign transaction
    transaction.sign(sourceKeys);
    
    // STEP 4: Submit to network
    const result = await server.submitTransaction(transaction);
    
    console.log('🎉 PAYMENT SUCCESSFUL!\n');
    console.log(`💰 Sent: ${amount} XLM`);
    console.log(`🔗 Hash: ${result.hash}\n`);
    
    return result;
    
  } catch (error) {
    console.error('❌ ERROR:', error.message);
    
    // Display detailed error information if available
    if (error.response && error.response.data) {
      console.error('Details:', JSON.stringify(error.response.data, null, 2));
    }
    
    throw error;
  }
}

/**
 * Main function to send payments to multiple recipients
 * Processes each payment and tracks results
 */
async function enviarPagosAutomatizados() {
  // Display header
  console.log('╔═══════════════════════════════════════════╗');
  console.log('  🚀 AUTOMATED PAYMENT SYSTEM');
  console.log('╚═══════════════════════════════════════════╝');
  
  // Array to store results of each transaction
  const resultados = [];
  
  // Process each recipient
  for (let i = 0; i < destinatarios.length; i++) {
    const dest = destinatarios[i];
    
    try {
      console.log(`\n[${i + 1}/${destinatarios.length}] Sending 2 XLM...`);
      
      // Attempt to send payment
      const resultado = await enviarPago(dest.publicKey, '2', dest.memo);
      
      // Store successful result
      resultados.push({
        destinatario: dest.publicKey,
        memo: dest.memo,
        hash: resultado.hash,
        estado: '✅ Successful'
      });
      
      console.log('✅ Transaction completed successfully');
      console.log('─────────────────────────────────────────');
      
    } catch (error) {
      // Log error but continue with next payment
      console.error(`❌ Error in ${dest.memo}:`, error.message);
      
      // Store failed result
      resultados.push({
        destinatario: dest.publicKey,
        memo: dest.memo,
        hash: 'N/A',
        estado: '❌ Failed'
      });
      
      console.log('─────────────────────────────────────────');
    }
  }
  
  // Display final summary
  console.log('\n╔═══════════════════════════════════════════╗');
  console.log('  📊 TRANSACTION SUMMARY');
  console.log('╚═══════════════════════════════════════════╝\n');
  
  // Show individual transaction results
  resultados.forEach((res, index) => {
    console.log(`${index + 1}. ${res.memo}`);
    console.log(`   Status:      ${res.estado}`);
    console.log(`   Recipient:   ${res.destinatario.substring(0, 8)}...`);
    console.log(`   Hash:        ${res.hash !== 'N/A' ? res.hash.substring(0, 16) + '...' : 'N/A'}\n`);
  });
  
  // Calculate statistics
  const exitosos = resultados.filter(r => r.estado === '✅ Successful').length;
  const fallidos = resultados.filter(r => r.estado === '❌ Failed').length;
  
  console.log(`✅ Successful: ${exitosos}`);
  console.log(`❌ Failed:     ${fallidos}`);
  console.log(`📊 Total:      ${resultados.length}`);
  console.log(`💰 XLM sent:   ${exitosos * 2} XLM\n`);
}

// Execute automated payment system
enviarPagosAutomatizados();