# Script para verificar balance del token BDB
$CONTRACT_ID = $env:VITE_CONTRACT_ID
$PUBLIC_KEY = $args[0]

stellar contract invoke `
    --id $CONTRACT_ID `
    --source-account sofia `
    --network testnet `
    -- balance `
    --id $PUBLIC_KEY