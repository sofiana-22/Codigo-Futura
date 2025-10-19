/**
 * ACCOUNT BALANCE MONITOR - ADVANCED VERSION
 * 
 * This script demonstrates how to:
 * 1. Monitor multiple Stellar accounts simultaneously
 * 2. Handle account not found errors gracefully
 * 3. Calculate aggregate statistics (total balance, trustlines)
 * 4. Display comprehensive monitoring dashboard
 * 
 * Use case: Portfolio monitoring, multi-account management
 * 
 * Usage: node ejercicios-avanzados/consultar-balance-monitor.js
 */

const StellarSdk = require('@stellar/stellar-sdk');

// Initialize Horizon server (testnet)
const server = new StellarSdk.Horizon.Server('https://horizon-testnet.stellar.org');

// Array of accounts to monitor
// Replace with your own accounts or load from environment variables
const PUBLIC_KEYS = [
  'GB5UVSQLG3R7WD76GFZKBNDI4JJ4BPBHD5PE4JJ56U7SP46NYGQ7M3DQ',
  'GCDIJ4WO6FF6XCZ5RDI5XGCVC6TFZA63EFLY5O6I4USJNFQLJ4OYVIC3',
  'GCQIVKWPLYGR3EK2JHUAJZIS4HMTO7FGDHQMUUE5SBHTWXXZQF7MZ7RK',
  'GDCV4NX7DT6YANGJPB3LAXMYRLKU4ZVXVQKVYZM2IRFX6M4W6KKB3K7E',
  'GDMMJR4A3DDPTNWY7ACIX5GYKZ6LPXABKDL465HEMJK4XYXRIL3WL7NL'
];

/**
 * Queries balance information for a single account
 * @param {string} publicKey - Stellar public key to query
 * @returns {Promise<object>} Account data or error information
 */
async function consultarBalance(publicKey) {
  try {
    // Load account data from blockchain
    const account = await server.loadAccount(publicKey);
    
    // Find XLM (native) balance
    const xlmBalance = account.balances.find(b => b.asset_type === 'native');
    const balance = xlmBalance ? parseFloat(xlmBalance.balance).toFixed(2) : '0.00';
    
    // Count trustlines (non-native assets)
    const trustlines = account.balances.filter(b => b.asset_type !== 'native').length;
    
    // Get sequence number (transaction counter)
    const sequenceNumber = account.sequenceNumber();
    
    return {
      publicKey: publicKey,
      balance: balance,
      trustlines: trustlines,
      sequenceNumber: sequenceNumber,
      error: null
    };
    
  } catch (error) {
    // Return error information instead of throwing
    return {
      publicKey: publicKey,
      balance: 'N/A',
      trustlines: 'N/A',
      sequenceNumber: 'N/A',
      error: error.response?.status === 404 ? 'Account not found' : error.message
    };
  }
}

/**
 * Main monitoring function for multiple accounts
 * Queries all accounts and displays comprehensive statistics
 * @param {string[]} publicKeys - Array of public keys to monitor
 */
async function monitorearCuentas(publicKeys) {
  // Display header
  console.log('╔═══════════════════════════════════════════╗');
  console.log('       🔍 STELLAR ACCOUNTS MONITOR');
  console.log('╚═══════════════════════════════════════════╝\n');
  
  const resultados = [];
  
  // Query each account sequentially
  for (let i = 0; i < publicKeys.length; i++) {
    console.log(`[${i + 1}/${publicKeys.length}] Querying account...`);
    
    const resultado = await consultarBalance(publicKeys[i]);
    resultados.push(resultado);
    
    // Display individual account result
    console.log(`\n=== ACCOUNT ${i + 1} ===`);
    
    // Show shortened public key for readability
    const shortKey = `${resultado.publicKey.substring(0, 5)}...${resultado.publicKey.substring(resultado.publicKey.length - 3)}`;
    console.log(`Account: ${shortKey}`);
    
    if (resultado.error) {
      // Display error message
      console.log(`  ❌ Error: ${resultado.error}`);
    } else {
      // Display account metrics
      console.log(`  💰 Balance:    ${resultado.balance} XLM`);
      console.log(`  🔗 Trustlines: ${resultado.trustlines}`);
      console.log(`  🔢 Sequence:   ${resultado.sequenceNumber}`);
    }
    
    console.log(''); // Blank line for readability
  }
  
  // Calculate and display aggregate statistics
  console.log('╔═══════════════════════════════════════════╗');
  console.log('       📊 MONITORING SUMMARY');
  console.log('╚═══════════════════════════════════════════╝\n');
  
  // Count active vs error accounts
  const cuentasActivas = resultados.filter(r => !r.error).length;
  const cuentasConError = resultados.filter(r => r.error).length;
  
  // Calculate total XLM balance across all active accounts
  const balanceTotal = resultados
    .filter(r => !r.error)
    .reduce((sum, r) => sum + parseFloat(r.balance), 0);
  
  // Calculate total trustlines across all active accounts
  const trustlinesTotal = resultados
    .filter(r => !r.error)
    .reduce((sum, r) => sum + r.trustlines, 0);
  
  // Display summary statistics
  console.log(`✅ Active accounts:    ${cuentasActivas}`);
  console.log(`❌ Accounts with errors: ${cuentasConError}`);
  console.log(`💰 Total balance:      ${balanceTotal.toFixed(2)} XLM`);
  console.log(`🔗 Total trustlines:   ${trustlinesTotal}`);
  console.log(`📊 Total monitored:    ${publicKeys.length} accounts`);
  
  // Display average balance per active account
  if (cuentasActivas > 0) {
    const avgBalance = balanceTotal / cuentasActivas;
    console.log(`📈 Average balance:    ${avgBalance.toFixed(2)} XLM`);
  }
  
  console.log(''); // Final blank line
}

// Execute the monitoring system
monitorearCuentas(PUBLIC_KEYS);