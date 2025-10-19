/**
 * CHECK BALANCE - BASIC VERSION
 * 
 * This script demonstrates how to:
 * 1. Query account information from the blockchain
 * 2. Display all asset balances
 * 3. Calculate available vs reserved XLM
 * 4. Handle errors (account not found)
 * 
 * Usage: node ejercicios-basicos/consultar-balance.js
 */

const StellarSdk = require('@stellar/stellar-sdk');

// Initialize Horizon server (testnet)
const server = new StellarSdk.Horizon.Server('https://horizon-testnet.stellar.org');

// Account to query - Replace with any Stellar public key
const PUBLIC_KEY = 'GAJBWEMXZW6I77IBUBLLIZZVOL73K2PD5XLCS3OCKP6QJG5RLJKL2W4Z';

/**
 * Queries and displays account balance information
 * @param {string} publicKey - Stellar public key to query
 * @returns {Promise<object>} Account object from Horizon
 */
async function consultarBalance(publicKey) {
  try {
    console.log(`🔍 Querying account: ${publicKey.substring(0, 8)}...\n`);
    
    // Load account data from the blockchain
    // This returns all account information including balances
    const account = await server.loadAccount(publicKey);
    
    // Display account header
    console.log('╔═══════════════════════════════════╗');
    console.log('📊 ACCOUNT INFORMATION');
    console.log('╚═══════════════════════════════════╝\n');
    
    // Account ID (same as public key)
    console.log(`📧 Account ID:`);
    console.log(`   ${account.id}\n`);
    
    // Sequence number - increases with each transaction
    // Used to prevent replay attacks
    console.log(`🔢 Sequence Number:`);
    console.log(`   ${account.sequenceNumber()}\n`);
    
    // Display all balances
    console.log('╔═══════════════════════════════════╗');
    console.log('💰 BALANCES');
    console.log('╚═══════════════════════════════════╝\n');
    
    // Loop through all assets held by this account
    account.balances.forEach((balance, index) => {
      if (balance.asset_type === 'native') {
        // Native asset = XLM (Lumens)
        console.log(`${index + 1}. 🌟 XLM (Lumens):`);
        console.log(`   Total: ${balance.balance} XLM`);
        
        // Calculate reserved XLM (cannot be spent)
        // Stellar requires minimum balance to keep account alive
        const baseReserve = 0.5; // Base reserve for account
        const subentryReserve = account.subentry_count * 0.5; // Reserve per subentry (trustlines, offers, etc.)
        const totalReserve = baseReserve + subentryReserve;
        const available = parseFloat(balance.balance) - totalReserve;
        
        console.log(`   Reserved: ${totalReserve.toFixed(7)} XLM`);
        console.log(`   Available: ${available.toFixed(7)} XLM\n`);
      } else {
        // Custom asset (token)
        console.log(`${index + 1}. 🪙 ${balance.asset_code}:`);
        console.log(`   Balance: ${balance.balance}`);
        console.log(`   Issuer: ${balance.asset_issuer.substring(0, 8)}...\n`);
      }
    });
    
    return account;
    
  } catch (error) {
    // Handle common errors
    if (error.response && error.response.status === 404) {
      console.error('❌ Account not found');
      console.log('💡 Possible causes:');
      console.log('   - Account was never created/funded');
      console.log('   - Typo in the public key');
      console.log('   - Wrong network (testnet vs mainnet)\n');
    } else {
      console.error('❌ Error:', error.message);
    }
    
    throw error;
  }
}

// Execute the query
consultarBalance(PUBLIC_KEY);