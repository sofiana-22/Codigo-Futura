<p align="center">
  <img src="../assets/tiburona.png" alt="Tiburona - Buen Día Builders" width="280">
</p>
# 🚀 Class 5 - ERC-20-Like Token Contract on Stellar / Soroban  
> Week 3 – “From Web2 Developer to Blockchain Builder”

---

## 🧠 Personal Summary

Today we learned how to **create, test, and deploy a complete token** similar to ERC-20 (CAP-46) on the **Stellar Soroban** network, using **Rust** and **WSL** as the main environment.

The professor strongly insisted on using **WSL instead of PowerShell**, because **the entire blockchain ecosystem is built for Linux**.  
Literally: *“PowerShell works, but WSL makes you part of the industry standard.”*

---

## ⚙️ Why WSL and not PowerShell?

💡 **Key idea:** almost all Soroban documentation and scripts are written for Bash/Linux.  
In PowerShell, there are syntax errors, incompatibilities, and more friction.

### Advantages of WSL
- Documentation 100% compatible with commands (`curl`, `grep`, `awk`, `sed`)
- Complete Linux ecosystem (apt, brew, Docker)
- Better performance and fewer weird errors
- Same environment as production servers (AWS, GCP, Azure)
- Smooth integration with VS Code

### Problems with pure PowerShell
- Different syntax (`$env:` vs `$`)
- Commands fail when copy/pasted from docs
- Community deploy scripts don’t run
- Lots of wasted time fixing issues

📌 *WSL = full fluency with the community and official documentation.*

---

## 🪄 Installing WSL (one time only)

```powershell
wsl --install
```

This installs:
- WSL 2  
- Ubuntu  
- Windows Terminal (optional)

Then restart the computer, create a user and password.  
The password only applies inside Ubuntu, not Windows.

### Quick verification

```bash
wsl --version
wsl -l -v   # Ubuntu Running (2)
```

Update packages:

```bash
sudo apt update && sudo apt upgrade -y
```

---

## 🧩 Environment setup

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
cargo --version
```

### 2. Install Stellar CLI

```bash
cargo install --locked stellar-cli
stellar --version   # 21.0.0 or higher
```

### 3. Add WASM target

```bash
rustup target add wasm32v1-none
rustup target list | grep wasm32
```

---

## 🗂️ Accessing Windows files

```bash
cd /mnt/c/Users/YourUser/Desktop
```

💬 *Tip:* you can type `wsl` directly in the Windows File Explorer bar to open it in that path.

---

## 💥 Deploying the BDB Token

1. **Generate account and identity**
   ```bash
   stellar keys generate alice --network testnet
   stellar keys address alice
   curl "https://friendbot.stellar.org?addr=$(stellar keys address alice)"
   stellar account balance alice --network testnet
   ```

2. **Compile the contract**
   ```bash
   cargo clean
   stellar contract build
   ls -lh target/wasm32v1-none/release/
   ```

3. **Deploy on testnet**
   ```bash
   stellar contract deploy      --wasm target/wasm32v1-none/release/token_bdb.wasm      --source alice      --network testnet
   ```

   Save the `CONTRACT_ID`:
   ```bash
   echo $CONTRACT_ID > .soroban/token_id
   ```

4. **Initialize the token**
   ```bash
   stellar contract invoke      --id $TOKEN_CONTRACT_ID      --source alice      --network testnet      -- initialize      --admin $(stellar keys address alice)      --name "Buen Dia Token"      --symbol "BDB"      --decimals 7
   ```

5. **Verify**
   ```bash
   stellar contract invoke --id $TOKEN_CONTRACT_ID --network testnet -- name
   stellar contract invoke --id $TOKEN_CONTRACT_ID --network testnet -- total_supply
   ```

---

## 💰 Basic operations

| Action | Key command |
|--------|--------------|
| **Mint** | `stellar contract invoke ... -- mint` |
| **Transfer** | `stellar contract invoke ... -- transfer` |
| **Approve** | `stellar contract invoke ... -- approve` |
| **Transfer with allowance** | `stellar contract invoke ... -- transfer_from` |
| **Burn tokens** | `stellar contract invoke ... -- burn` |

### Transfer example
```bash
stellar contract invoke   --id $TOKEN_CONTRACT_ID   --source alice   --network testnet   -- transfer   --from $(stellar keys address alice)   --to $(stellar keys address bob)   --amount 1000000000
```

---

## 🧠 Important concepts

### Stroops
> 1 token = 10^decimals stroops  
If `decimals = 7`, then:  
1 token = 10,000,000 stroops.

---

## 🪄 Automatic deploy script

File `deploy.sh`:
```bash
#!/bin/bash
set -e
stellar contract build
CONTRACT_ID=$(stellar contract deploy   --wasm target/wasm32v1-none/release/token_bdb.wasm   --source alice   --network testnet)
echo $CONTRACT_ID > .soroban/token_id
```

Run:
```bash
chmod +x deploy.sh
./deploy.sh
```

---

## 🧩 Contract Structure (Rust)

```rust
pub trait TokenTrait {
    fn initialize(env: Env, admin: Address, name: String, symbol: String, decimals: u32);
    fn mint(env: Env, to: Address, amount: u128) -> Result<(), TokenError>;
    fn transfer(env: Env, from: Address, to: Address, amount: u128) -> Result<(), TokenError>;
    fn burn(env: Env, from: Address, amount: u128) -> Result<(), TokenError>;
    fn approve(env: Env, from: Address, spender: Address, amount: u128) -> Result<(), TokenError>;
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: u128) -> Result<(), TokenError>;
}
```

📌 Implements methods from the **CAP-46** standard, similar to ERC-20 but adapted to the **Stellar Soroban** environment.

---

## 🧰 Quick Troubleshooting

| Error | Solution |
|-------|-----------|
| `Contract already initialized` | Deploy a new contract |
| `command not found: stellar` | Run `source $HOME/.cargo/env` |
| Variables disappear | Load with `export TOKEN_CONTRACT_ID=$(cat .soroban/token_id)` |

---

## 🦈 Senior Shark Tips

- ✅ Always check if the contract is initialized before using it.  
- 🧾 Use **events** for debugging (`TransferEvent`, `MintEvent`, etc.).  
- ⚠️ Watch for *overflow* using `checked_add` and `checked_sub`.  
- 🧠 Store global counters (e.g., `HolderCount`) in **instance**, not in persistent.  
- 🕒 Remember to extend **TTL** on every operation that modifies storage.  
- 🧪 Test edge cases: self-transfers, large balances, invalid allowances.

---

## 📘 Final Summary

✅ I learned to:
- Use **WSL** as a professional environment for blockchain  
- **Compile and deploy** Soroban contracts on testnet  
- Create **complete tokens** (mint, burn, transfer, approve…)  
- Use **Rust** with secure coding practices  
- **Automate deployment** with Bash scripts

📅 Next step → **Class 6: Frontend Integration (React + Freighter Wallet)**
