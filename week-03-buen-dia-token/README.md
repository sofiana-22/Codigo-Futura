# 🦈 Week 3 — My Token Comes to Life

> “From writing code to seeing my token live on the blockchain.”  
> This week was the biggest leap so far: I went from Rust code lines to a real application running on Stellar Testnet, with my wallet connected and a frontend I built myself.

---

## 🚀 Week Summary

This was a total transition week.  
I left the theoretical part behind and dove straight into the **real world of blockchain development** — compiling, deploying, connecting, and seeing my contract actually work.

For the first time, **my token left the code and lived on the network**, and I managed to make my frontend talk to it like a real app.

---

## 💡 What I Achieved

- Compiled my Rust contract to the correct WASM target (`wasm32v1-none`)
- Deployed my token to **Stellar Testnet**
- Connected my account and wallet through Freighter
- Created a **fully functional frontend using Scaffold Stellar**
- Read and invoked contract functions directly from the UI
- Minted and saw my first BDB tokens in action ✨

---

## 🧭 Key Learnings

- **Blockchain ≠ magic:** it’s all code, connections, and digital signatures.
- **The `Contract ID` is basically my backend address**, the bridge that connects everything.
- **The wallet isn’t just storage** — it’s my digital signature for every transaction.
- Errors are part of the process (and usually make sense once I actually read them 😅).
- **Each command brings me closer to production.**

---

## ⚙️ Key Technical Moments

```bash
# Compile my contract
stellar contract build

# Deploy to testnet
stellar contract deploy --wasm optimized.wasm --source testnet --network testnet

# Generate a Stellar-ready frontend
stellar scaffold init mi-token-bdb

# Connect wallet in UI
npm i @stellar/freighter-api @stellar/stellar-sdk

# Read contract balance
stellar contract invoke --id CXXXX... --network testnet -- balance --id GXXXX...
```

Each of these commands felt like a mini victory. I went from testing on localhost to seeing **real data on the blockchain** 🫶

---

## 🧠 Challenges & Lessons

- I struggled to understand why the `wasm32v1-none` target mattered (the old one doesn’t work for Soroban).  
- Freighter popups were blocked at first 😅  
- I learned that every CLI error happens for a reason — reading it carefully changes everything.  
- And above all: **don’t fear deployment**. The fear fades once it works.

---

## 🌊 Builder Mindset of the Week

This week I truly felt like I’m *building for real*.  
Not just following steps, but **connecting dots** I now understand — Rust, blockchain, frontend, wallet, network.

I learned that being a builder isn’t about knowing everything; it’s about **daring to try**, breaking things, and fixing them without giving up.

---

## 🔮 Next Steps

- Improve my UI and add token transfer functionality  
- Customize my BDB token’s visual identity  
- Prepare for **Product Quest on Saturday** (my idea is already forming!)

---

## ✨ Final Reflection

A few weeks ago I didn’t know what a smart contract was.  
Today I can say: **my contract lives on a public blockchain — and I built it.**

Each line of code brings me closer to becoming a real blockchain builder.

> _“Let’s keep building, Tiburonas.”_ 💙🌊
