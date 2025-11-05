import * as SorobanClient from 'soroban-client';

const contractId = import.meta.env.VITE_CONTRACT_ID;
const rpcUrl = import.meta.env.VITE_STELLAR_RPC_URL || 'https://soroban-testnet.stellar.org';

const server = new SorobanClient.Server(rpcUrl);

export async function getTokenBalance(accountAddress: string): Promise<string> {
  try {
    console.log('🔍 Obteniendo balance para:', accountAddress);
    console.log('📋 Contract ID:', contractId);
    
    // Crear el contrato
    const contract = new SorobanClient.Contract(contractId);
    
    // Obtener la cuenta
    const account = await server.getAccount(accountAddress);
    
    // Crear el argumento de dirección
    const addressArg = SorobanClient.Address.fromString(accountAddress).toScVal();
    
    // Construir transacción
    const transaction = new SorobanClient.TransactionBuilder(account, {
      fee: SorobanClient.BASE_FEE,
      networkPassphrase: SorobanClient.Networks.TESTNET,
    })
      .addOperation(contract.call('balance', addressArg))
      .setTimeout(30)
      .build();
    
    // Simular
    const response = await server.simulateTransaction(transaction);
    
    console.log('📡 Respuesta:', response);
    
    if (response.results && response.results.length > 0) {
      const resultValue = response.results[0].xdr;
      const scVal = SorobanClient.xdr.ScVal.fromXDR(resultValue, 'base64');
      const balance = scVal.value().toString();
      
      console.log('✅ Balance:', balance);
      return balance;
    }
    
    throw new Error('No se pudo obtener el balance');
    
  } catch (error: any) {
    console.error('❌ Error:', error);
    
    if (error.message?.includes('Account not found') || error.message?.includes('actNotFound')) {
      console.log('ℹ️ Cuenta no encontrada, balance = 0');
      return '0';
    }
    
    throw error;
  }
}