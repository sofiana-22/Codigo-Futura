# 🚀 Class 6 — My Token Comes to Life (Student Notes)

> Goal of the day: see my BDB token **alive** on testnet and interact with it from a **frontend** using my **wallet**. No extra theory — just the minimum steps to make it work.

---

## 0) Quick Checklist Before Starting
- Node v22 `node -v`
- Stellar CLI 22+ `stellar --version`
- Rust + wasm target `rustup target list | grep wasm32v1-none`
- Scaffold plugin (if it fails, install it) `stellar install scaffold`

> If something is missing:  
> - Node: `nvm install 22 && nvm use 22`  
> - CLI: `cargo install --locked stellar-cli`  
> - Target: `rustup target add wasm32v1-none`

---

## 1) Mental Flow (to stay oriented)
**Rust → WASM → Deploy (Contract ID) → Frontend → Wallet → Invoke functions**  
Think of `Contract ID` as “the address of my backend” and the wallet as the **signature** for my actions.

---

## 2) Deploy to Testnet (practical summary)
1. Be inside the contract folder.  
2. Compile to the correct target:  
   ```bash
   stellar contract build
   ```
3. (Optional) Optimize size:  
   ```bash
   wasm-opt -Oz -o optimized.wasm target/wasm32v1-none/release/MY_CONTRACT.wasm
   ```
4. Check testnet account/funds:  
   ```bash
   stellar keys generate testnet --network testnet --fund   # only once
   stellar keys address testnet
   ```
5. Deploy (use optimized wasm if available):  
   ```bash
   stellar contract deploy      --wasm optimized.wasm      --source testnet      --network testnet
   ```
6. **Save your `CONTRACT_ID`** like gold. Write it in `.env.local`, a post‑it, and this file:  
   - `CONTRACT_ID = CXXXXXXXXXXXXXXXXXXXXXXXX`

7. Quick “alive test”:  
   ```bash
   stellar contract invoke --id $CONTRACT_ID --network testnet -- --help
   ```

**If it fails:** it’s usually the wrong target (don’t use `wasm32-unknown-unknown`), missing CLI, or no funds.

---

## 3) Frontend with Scaffold (the minimum to see it working)
1. Create the project:
   ```bash
   stellar scaffold init my-token-bdb
   cd my-token-bdb
   ```
2. Place your contract inside `contracts/` (copy the contract folder).  
3. Environment variables (`.env`):
   ```bash
   VITE_STELLAR_NETWORK=testnet
   VITE_STELLAR_RPC_URL=https://soroban-testnet.stellar.org
   VITE_BDB_CONTRACT_ID=CXXXXXXXXXXXXXXXXXXXXXXXX
   ```
4. Install dependencies:
   ```bash
   npm install
   npm i @stellar/freighter-api @stellar/stellar-sdk
   npm run build:contracts   # generates the TypeScript client from my contract
   ```
5. Run the project:
   ```bash
   npm run dev
   # Open http://localhost:5173
   ```

> If `packages/...` doesn’t appear: recompile the contract in `contracts/` and rerun `npm run build:contracts`.

---

## 4) Wallet (Freighter) and first “ping” to the contract
- Use Chrome/Firefox. In Freighter, **network: Testnet**.  
- Import my testnet account if needed (**NEVER paste your secret key in any code!**)  
- Connect the wallet from the frontend button (popup).  
- To read my BDB balance from the app:
  - Import the generated client (something like `BuenDiaTokenClient`), pass it the `contractId`, `rpcUrl`, and **testnet network passphrase** (`Test SDF Network ; September 2015`), then call `balance({ id: <my G...> })`.
- If it shows `0`, that’s normal. You can mint a test amount and recheck:
  ```bash
  stellar contract invoke     --id $CONTRACT_ID --source testnet --network testnet     -- mint --to MY_PUBLIC_KEY --amount 10000000   # if token has 7 decimals, this equals 1.0
  ```

**Disaster rule:** everything starting with **S** (secret key/seed) is private. Never in repos, never in .env, never in screenshots.

---

## 5) Common Errors (and How I Solved Them)
- **`command not found: stellar`** → missing Rust env or CLI.  
  `source $HOME/.cargo/env && cargo install --locked stellar-cli`
- **“Invalid WASM module / size too big”** → outdated or unoptimized target.  
  `rustup target add wasm32v1-none` + `wasm-opt -Oz`
- **TypeScript client not found** → contract not inside `contracts/` or failed build.  
  Recompile the contract, delete `packages/`, and rerun `npm run build:contracts`.
- **Blank frontend** → incorrect `.env` (missing `VITE_` prefix) or didn’t restart `npm run dev` after editing it.
- **Freighter not opening** → unlock extension, disable blockers, or try incognito.
- **Weird balance (10000000 instead of 1)** → remember **decimals**. Format as `Number(raw)/10**decimals`.

---

## 6) Mini‑Glossary (human version)
- **WASM:** “portable binary” of the contract. It’s what goes on-chain.
- **Contract ID (C…):** unique contract address. Used by the frontend to communicate.
- **Network passphrase:** identifies the network (Testnet/Mainnet). If wrong, nothing works.
- **Public key (G…):** my public address (safe to share).
- **Secret key (S…):** my private key (NEVER share).

---

## 7) Commands I Actually Used Today
```bash
# Compile
stellar contract build

# Optimize (optional)
wasm-opt -Oz -o optimized.wasm target/wasm32v1-none/release/MY_CONTRACT.wasm

# Account and testnet funds
stellar keys generate testnet --network testnet --fund
stellar keys address testnet

# Deploy
stellar contract deploy --wasm optimized.wasm --source testnet --network testnet

# View available functions
stellar contract invoke --id $CONTRACT_ID --network testnet -- --help

# Read balance / Mint
stellar contract invoke --id $CONTRACT_ID --network testnet -- balance --id MY_PUBLIC_KEY
stellar contract invoke --id $CONTRACT_ID --source testnet --network testnet -- mint --to MY_PUBLIC_KEY --amount 10000000

# Scaffold
stellar scaffold init my-token-bdb
npm i
npm i @stellar/freighter-api @stellar/stellar-sdk
npm run build:contracts
npm run dev
```

---

## 8) My Personal Closing Checklist
- [ ] Saved `CONTRACT_ID` in `.env` and notebook  
- [ ] Frontend loads and wallet connects (popup approved)  
- [ ] `balance()` responds (even if 0)  
- [ ] Successfully minted and saw the number change  
- [ ] No **secret keys** in project files  
- [ ] Wrote down errors and how I fixed them (so I don’t repeat them)

---

## 9) Things to Improve Later (optional task)
- Format amounts using `decimals` (show 1.23 instead of 12300000).  
- Add **transfer** form (with 56‑char G… validation).  
- Small “dashboard” showing total supply and my balance.  
- Dark mode and better styles.  
- Deploy frontend on Vercel/Netlify (frontend only).

---

### Final Note (for myself)
If I get stuck: **read full error messages**, compare with this guide, and ask for help with details (exact command + error). 80% of issues were due to the WASM target, `.env` misconfiguration, or Freighter being blocked.
