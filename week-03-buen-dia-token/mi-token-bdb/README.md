# 💜 Mi Token BDB - Frontend

Frontend interactivo para el token BDB (Buen Día Token) en Stellar Testnet.

Desarrollado por **Sofía the Shark** 🦈 como parte del bootcamp Código Futura.

## ✨ Features

- 🔐 **Conexión con Freighter Wallet** - Integración completa con la wallet de Stellar
- 💰 **Balance en tiempo real** - Consulta tu balance directamente desde la blockchain
- 💸 **Formulario de transferencia** - UI completa para transferir tokens (requiere firma)
- 🌓 **Modo oscuro/claro** - Toggle entre temas con persistencia en localStorage
- 💜 **Diseño personalizado** - Paleta de colores lila/púrpura con transiciones suaves
- 📱 **Responsive** - Funciona en desktop y mobile

## 🛠️ Tecnologías

- **React 18** - Framework de UI
- **TypeScript** - Tipado estático
- **Vite** - Build tool y dev server
- **@stellar/freighter-api** - Integración con Freighter Wallet
- **Express** - Backend para ejecutar comandos de Stellar CLI
- **Stellar SDK** - Interacción con Stellar blockchain

## 🚀 Setup Local

### Prerrequisitos

- Node.js v22+
- Freighter Wallet instalada y configurada en TESTNET
- Cuenta con XLM testnet

### Instalación

1. **Clonar el repositorio:**
```bash
git clone [URL_DEL_REPO]
cd week-03-buen-dia-token/mi-token-bdb
```

2. **Instalar dependencias:**
```bash
npm install
```

3. **Configurar variables de entorno:**
```bash
cp .env.example .env
```

Editar `.env` y agregar tu Contract ID:
```
VITE_STELLAR_NETWORK=testnet
VITE_STELLAR_RPC_URL=https://soroban-testnet.stellar.org
VITE_CONTRACT_ID=TU_CONTRACT_ID_AQUI
```

4. **Configurar backend (opcional para transferencias):**

Crear `server.cjs` con tu configuración de backend.

5. **Arrancar desarrollo:**

**Terminal 1 - Frontend:**
```bash
npm run dev
```

**Terminal 2 - Backend (opcional):**
```bash
node server.cjs
```

6. **Abrir en navegador:**
```
http://localhost:5173
```

## 📖 Uso

1. **Conectar Wallet:**
   - Click en "🔐 Conectar Freighter Wallet"
   - Aprobar la conexión en Freighter

2. **Ver Balance:**
   - Click en "💰 Ver mi Balance BDB REAL"
   - Espera 5-10 segundos mientras consulta la blockchain

3. **Cambiar Tema:**
   - Click en el botón 🌙/☀️ arriba a la derecha

4. **Transferir (requiere configuración adicional):**
   - Ingresar dirección destino (empieza con G)
   - Ingresar cantidad en BDB
   - Click en "💸 Transferir BDB"

## 🏗️ Estructura del Proyecto
```
mi-token-bdb/
├── src/
│   ├── App.tsx          # Componente principal con toda la lógica
│   ├── App.css          # Estilos
│   └── main.tsx         # Entry point
├── public/              # Assets estáticos
├── contracts/           # Contratos Soroban (referencia)
├── .env.example         # Template de variables de entorno
├── server.cjs           # Backend Express (no incluido en repo)
├── package.json         # Dependencias
└── vite.config.ts       # Configuración de Vite
```

## 🎨 Temas

### Modo Claro
- Fondo: Blanco
- Primario: Lila (`#9b59b6`)
- Secundario: Lila claro (`#bb86fc`)

### Modo Oscuro
- Fondo: Azul oscuro (`#1a1a2e`)
- Primario: Lila claro (`#bb86fc`)
- Secundario: Púrpura (`#9b59b6`)

## 🔐 Seguridad

- ✅ Las secret keys NUNCA se incluyen en el código
- ✅ `.env` está en `.gitignore`
- ✅ Todas las transacciones requieren aprobación en Freighter
- ✅ Solo funciona en TESTNET (no hay riesgo de fondos reales)

## 📝 Notas

- El backend (`server.cjs`) ejecuta comandos de Stellar CLI y no está incluido en el repositorio por seguridad
- Para transferencias con firma de usuario, se requiere integración adicional con `@stellar/stellar-sdk`
- El formulario de transferencia está implementado (UI) pero requiere firma desde el navegador

## 🦈 Sobre el Proyecto

Proyecto desarrollado como parte del bootcamp **Código Futura** - Semana 3: Smart Contracts en Soroban.

El objetivo fue crear un token ERC-20 compatible en Stellar y construir un frontend completo para interactuar con él.

**Desarrollado con 💜 por Sofía**

Powered by Stellar Blockchain 🚀