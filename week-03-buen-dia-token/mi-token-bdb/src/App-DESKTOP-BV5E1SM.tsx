import { useState } from 'react'
import { isConnected, getPublicKey, requestAccess } from '@stellar/freighter-api'
import './App.css'

function App() {
  const [publicKey, setPublicKey] = useState<string>('')
  const [connected, setConnected] = useState<boolean>(false)
  const [balance, setBalance] = useState<string>('0')
  const [loading, setLoading] = useState<boolean>(false)
  const [toAddress, setToAddress] = useState<string>('')
  const [amount, setAmount] = useState<string>('')
  const [transferring, setTransferring] = useState<boolean>(false)
  const [darkMode, setDarkMode] = useState<boolean>(() => {
    const saved = localStorage.getItem('darkMode')
    return saved === 'true'
  })

  const connectWallet = async () => {
    try {
      console.log('1️⃣ Verificando si Freighter está instalado...')
      
      if (await isConnected()) {
        console.log('2️⃣ Freighter está instalado')
        console.log('3️⃣ Solicitando acceso...')
        
        const accessGranted = await requestAccess()
        console.log('4️⃣ Acceso concedido:', accessGranted)
        
        if (!accessGranted) {
          alert('❌ Necesitás aprobar el acceso en Freighter')
          return
        }
        
        console.log('5️⃣ Obteniendo public key...')
        const pk = await getPublicKey()
        
        console.log('🔑 Public key recibida:', pk)
        console.log('🔑 Tipo:', typeof pk)
        console.log('🔑 Longitud:', pk?.length)
        
        if (!pk || pk.length === 0) {
          alert('❌ Freighter devolvió una public key vacía. Asegúrate de:\n\n1. Estar en TESTNET\n2. Tener una cuenta activa en Freighter\n3. Tener XLM en la cuenta')
          return
        }
        
        setPublicKey(pk)
        setConnected(true)
        
        console.log('✅ Wallet conectada exitosamente')
      } else {
        alert('❌ Por favor instalá Freighter wallet desde https://freighter.app')
      }
    } catch (error: any) {
      console.error('💥 Error completo:', error)
      alert('❌ Error al conectar: ' + error.message)
    }
  }

  const getBalance = async () => {
    console.log('🔍 Estado actual - connected:', connected)
    console.log('🔍 Estado actual - publicKey:', publicKey)
    
    if (!connected) {
      alert('⚠️ Conectá tu wallet primero')
      return
    }

    if (!publicKey || publicKey.length === 0) {
      alert('⚠️ Public key vacía. Reconectá la wallet.')
      return
    }

    setLoading(true)
    
    try {
      console.log('📞 Llamando al backend con publicKey:', publicKey)
      
      const response = await fetch('http://localhost:3001/api/balance', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ publicKey: publicKey })
      })

      const data = await response.json()
      console.log('📦 Data recibida:', data)
      
      if (response.ok) {
        const balanceStr = String(data.balance).replace(/['"]/g, '')
        console.log('✅ Balance parseado:', balanceStr)
        setBalance(balanceStr)
      } else {
        throw new Error(data.error || 'Error desconocido')
      }
      
    } catch (error: any) {
      console.error('❌ Error:', error)
      alert(`❌ Error: ${error.message}`)
    } finally {
      setLoading(false)
    }
  }

  const transferTokens = async () => {
    if (!toAddress || !amount) {
      alert('⚠️ Por favor ingresá dirección destino y cantidad')
      return
    }

    if (!toAddress.startsWith('G') || toAddress.length !== 56) {
      alert('❌ Dirección inválida. Debe empezar con G y tener 56 caracteres.')
      return
    }

    const amountNum = Number(amount)
    if (isNaN(amountNum) || amountNum <= 0) {
      alert('❌ Cantidad inválida')
      return
    }

    const amountInStroops = Math.floor(amountNum * 10000000)

    if (window.confirm(`¿Confirmar transferencia de ${amount} BDB a ${toAddress.slice(0, 8)}...${toAddress.slice(-8)}?`)) {
      setTransferring(true)

      try {
        console.log('💸 Iniciando transferencia...')

        const response = await fetch('http://localhost:3001/api/transfer', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({
            from: publicKey,
            to: toAddress,
            amount: amountInStroops
          })
        })

        const data = await response.json()

        if (response.ok) {
          alert('✅ ¡Transferencia exitosa!')
          console.log('✅ Transfer completado:', data)
          
          setToAddress('')
          setAmount('')
          getBalance()
        } else {
          throw new Error(data.error || 'Error en transferencia')
        }

      } catch (error: any) {
        console.error('❌ Error:', error)
        alert(`❌ Error: ${error.message}`)
      } finally {
        setTransferring(false)
      }
    }
  }

  const toggleDarkMode = () => {
    const newMode = !darkMode
    setDarkMode(newMode)
    localStorage.setItem('darkMode', String(newMode))
  }

  const themes = {
    light: {
      bg: '#ffffff',
      bgSecondary: '#f3e5f5',
      primary: '#9b59b6',
      secondary: '#bb86fc',
      text: '#333333',
      textSecondary: '#666666',
      border: '#bb86fc',
      card: '#f3e5f5'
    },
    dark: {
      bg: '#1a1a2e',
      bgSecondary: '#16213e',
      primary: '#bb86fc',
      secondary: '#9b59b6',
      text: '#e0e0e0',
      textSecondary: '#a0a0a0',
      border: '#9b59b6',
      card: '#0f3460'
    }
  }

  const theme = darkMode ? themes.dark : themes.light

  return (
    <div className="App" style={{ 
      padding: '40px', 
      maxWidth: '600px', 
      margin: '0 auto',
      backgroundColor: theme.bg,
      color: theme.text,
      minHeight: '100vh',
      transition: 'all 0.3s ease'
    }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '10px' }}>
        <h1 style={{ color: theme.primary, fontSize: '48px', margin: '0' }}>
          💜 My Token BDB
        </h1>
        
        <button
          onClick={toggleDarkMode}
          style={{
            padding: '10px 20px',
            fontSize: '24px',
            backgroundColor: theme.card,
            color: theme.text,
            border: '2px solid ' + theme.border,
            borderRadius: '50px',
            cursor: 'pointer',
            transition: 'all 0.3s ease'
          }}
        >
          {darkMode ? '☀️' : '🌙'}
        </button>
      </div>

      <p style={{ color: theme.secondary, fontSize: '16px', fontWeight: 'bold', marginTop: '0' }}>
        by Sofía the shark 🦈
      </p>
      <p style={{ color: theme.textSecondary, fontSize: '14px' }}>Connect your Freighter Wallet to interact</p>
      
      {!connected ? (
        <div>
          <p>🔌 Conectá tu wallet para ver tu balance de tokens BDB</p>
          
          <button 
            onClick={connectWallet}
            style={{
              padding: '12px 24px',
              fontSize: '16px',
              backgroundColor: theme.primary,
              color: 'white',
              border: 'none',
              borderRadius: '8px',
              cursor: 'pointer',
              marginTop: '20px',
              fontWeight: 'bold'
            }}
          >
            🔐 Conectar Freighter Wallet
          </button>

          <div style={{ marginTop: '20px', fontSize: '12px', color: theme.textSecondary }}>
            <p>ℹ️ Asegurate de:</p>
            <ul style={{ textAlign: 'left', lineHeight: '1.8' }}>
              <li>Tener Freighter instalado</li>
              <li>Estar en TESTNET</li>
              <li>Tener la wallet desbloqueada</li>
            </ul>
          </div>
        </div>
      ) : (
        <div>
          <div style={{ 
            padding: '20px', 
            backgroundColor: theme.bgSecondary,
            borderRadius: '8px',
            marginTop: '20px',
            border: '2px solid ' + theme.border
          }}>
            <p style={{ fontWeight: 'bold', margin: '0 0 10px 0' }}>
              ✅ Wallet Conectada
            </p>
            <p style={{ fontSize: '12px', color: theme.textSecondary, margin: '0 0 8px 0' }}>
              Tu dirección:
            </p>
            <code style={{ 
              backgroundColor: theme.bg, 
              padding: '10px', 
              borderRadius: '4px',
              display: 'block',
              wordBreak: 'break-all',
              fontSize: '11px',
              border: '1px solid ' + theme.textSecondary,
              color: theme.text
            }}>
              {publicKey}
            </code>
          </div>

          <div style={{ marginTop: '30px' }}>
            <button 
              onClick={getBalance}
              disabled={loading}
              style={{
                padding: '12px 24px',
                fontSize: '16px',
                backgroundColor: loading ? '#ccc' : theme.secondary,
                color: 'white',
                border: 'none',
                borderRadius: '8px',
                cursor: loading ? 'not-allowed' : 'pointer',
                fontWeight: 'bold',
                width: '100%'
              }}
            >
              {loading ? '⏳ Consultando blockchain...' : '💰 Ver mi Balance BDB REAL'}
            </button>

            <div style={{
              marginTop: '20px',
              padding: '30px',
              backgroundColor: theme.bgSecondary,
              borderRadius: '8px',
              textAlign: 'center',
              border: '2px solid ' + theme.border
            }}>
              <p style={{ fontSize: '14px', margin: '0 0 12px 0', color: theme.textSecondary }}>
                💎 Balance en Testnet:
              </p>
              <p style={{ 
                fontSize: '64px', 
                fontWeight: 'bold', 
                margin: '0',
                color: theme.primary,
                lineHeight: '1'
              }}>
                {balance && !isNaN(Number(balance)) && Number(balance) > 0
                  ? (Number(balance) / 10000000).toFixed(2)
                  : '0.00'
                }
              </p>
              <p style={{ fontSize: '32px', color: theme.secondary, margin: '8px 0 0 0', fontWeight: 'bold' }}>
                BDB
              </p>
              <p style={{ fontSize: '11px', color: theme.textSecondary, marginTop: '15px', fontStyle: 'italic' }}>
                {balance && !isNaN(Number(balance)) && Number(balance) > 0
                  ? `(${Number(balance).toLocaleString()} stroops)`
                  : '(Consultá tu balance para ver tokens)'
                }
              </p>
            </div>
          </div>

          {/* FORMULARIO DE TRANSFERENCIA */}
          <div style={{ marginTop: '40px', padding: '20px', backgroundColor: theme.bgSecondary, borderRadius: '8px', border: '2px solid ' + theme.border }}>
            <h3 style={{ margin: '0 0 15px 0', color: theme.primary }}>💸 Transferir BDB</h3>
            
            <div style={{ marginBottom: '15px' }}>
              <label style={{ display: 'block', fontSize: '12px', color: theme.textSecondary, marginBottom: '5px' }}>
                Dirección destino:
              </label>
              <input 
                type="text" 
                placeholder="G..." 
                value={toAddress}
                onChange={(e) => setToAddress(e.target.value)}
                style={{
                  width: '100%',
                  padding: '10px',
                  borderRadius: '4px',
                  border: '1px solid ' + theme.border,
                  fontSize: '12px',
                  boxSizing: 'border-box',
                  backgroundColor: theme.bg,
                  color: theme.text
                }}
              />
            </div>
            
            <div style={{ marginBottom: '15px' }}>
              <label style={{ display: 'block', fontSize: '12px', color: theme.textSecondary, marginBottom: '5px' }}>
                Cantidad (BDB):
              </label>
              <input 
                type="number" 
                placeholder="10" 
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                min="0"
                step="0.1"
                style={{
                  width: '100%',
                  padding: '10px',
                  borderRadius: '4px',
                  border: '1px solid ' + theme.border,
                  fontSize: '14px',
                  boxSizing: 'border-box',
                  backgroundColor: theme.bg,
                  color: theme.text
                }}
              />
              <p style={{ fontSize: '10px', color: theme.textSecondary, margin: '5px 0 0 0' }}>
                Balance disponible: {(Number(balance) / 10000000).toFixed(2)} BDB
              </p>
            </div>
            
            <button 
              onClick={transferTokens}
              disabled={transferring}
              style={{
                padding: '12px 24px',
                fontSize: '16px',
                backgroundColor: transferring ? '#ccc' : theme.primary,
                color: 'white',
                border: 'none',
                borderRadius: '8px',
                cursor: transferring ? 'not-allowed' : 'pointer',
                fontWeight: 'bold',
                width: '100%'
              }}
            >
              {transferring ? '⏳ Transfiriendo...' : '💸 Transferir BDB'}
            </button>
          </div>

          <div style={{ 
            marginTop: '40px', 
            padding: '20px',
            textAlign: 'center',
            borderTop: '1px solid ' + theme.border
          }}>
            <p style={{ fontSize: '12px', color: theme.primary, margin: '0' }}>
              💜 Made with love by Sofia
            </p>
            <p style={{ fontSize: '10px', color: theme.secondary, margin: '5px 0 0 0' }}>
              Powered by Stellar Blockchain 🦈
            </p>
          </div>
        </div>
      )}

      <div style={{ 
        marginTop: '40px', 
        fontSize: '11px', 
        color: theme.textSecondary,
        padding: '15px',
        backgroundColor: theme.bgSecondary,
        borderRadius: '8px'
      }}>
        <p style={{ margin: '5px 0' }}>📋 Contract ID: {import.meta.env.VITE_CONTRACT_ID?.slice(0, 30)}...</p>
        <p style={{ margin: '5px 0' }}>🌐 Network: {import.meta.env.VITE_STELLAR_NETWORK}</p>
      </div>
    </div>
  )
}

export default App