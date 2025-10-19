\# Class 2: First Blockchain Code



\*\*Date:\*\* October 9, 2025  

\*\*Duration:\*\* 90 minutes  

\*\*Focus:\*\* Writing actual code - no more clicking



---



\## The Big Shift



\*\*Class 1:\*\* Used web tools with clicks (felt like a user)  

\*\*Class 2:\*\* Wrote code that controls money on real blockchain (feel like a developer)



---



\## What I Built Today



\### 1. JavaScript + Stellar SDK



\*\*Installed:\*\*

```bash

node --version  # v18+

npm --version

npm install stellar-sdk

```



\*\*Three scripts I wrote:\*\*



\*\*create-account.js\*\*

\- Generates cryptographically secure keypairs

\- Funds accounts with Friendbot

\- All automatic, no clicks



\*\*send-payment.js\*\*

\- Sends payments with pure JavaScript

\- No Laboratory, no wallet interface

\- Just code



\*\*check-balance.js\*\*

\- Queries any account balance

\- Ready to automate

\- Works in seconds



\### 2. Terminal \& Stellar CLI



\*\*Basic commands I learned:\*\*

```bash

cd folder-name        # Change directory

ls                    # List files

node script.js        # Run JavaScript

```



\*\*Stellar CLI:\*\*

```bash

stellar keys generate myidentity

stellar keys address myidentity

stellar keys fund myidentity --network testnet

stellar balance

```



Generated my CLI identity in one command - no Freighter needed.



\### 3. My First Smart Contract Deployment! 🚀



\*\*What I did:\*\*

1\. Downloaded a pre-compiled WASM contract

2\. Deployed it to testnet blockchain

3\. Got my Contract ID

4\. Invoked the `hello` function

5\. Verified on StellarExpert



\*\*This contract is LIVE on blockchain. Anyone can use it.\*\*



---



\## New Technical Concepts



\### JavaScript Concepts



\*\*import/export:\*\* How to use libraries

```javascript

import { Keypair, Server } from 'stellar-sdk';

```



\*\*async/await:\*\* Wait for blockchain operations

```javascript

const account = await server.loadAccount(publicKey);

```



\*\*try/catch:\*\* Handle errors gracefully

```javascript

try {

&nbsp; // code

} catch (error) {

&nbsp; console.error(error);

}

```



\### Smart Contract Concepts



\*\*WASM:\*\* Compiled format that runs on blockchain  

\*\*Deploy:\*\* Upload contract to network  

\*\*Invoke:\*\* Call contract functions  

\*\*Contract ID:\*\* Unique address of the contract (like account address but for contracts)



---



\## My Evolution



```

BEFORE CLASS 2          AFTER CLASS 2

──────────────         ─────────────

🖱️  Clicking           💻  Coding

👀  Watching           🔨  Building

📱  Using apps         🚀  Creating apps

```



---



\## Real-World Impact I Understood



\*\*Example they gave:\*\*

My cousin in Buenos Aires sells crafts online. Customers in Mexico, Chile, Colombia.



\*\*Problem:\*\*

\- Banks take days

\- Charge 5-10% commission  

\- Closed on weekends



\*\*My solution (the script I wrote today):\*\*

\- Works 24/7

\- Takes 5 seconds

\- Costs $0.00001

\- No permissions needed



\*\*The code I wrote today solves real problems.\*\*



---



\## What I Actually Accomplished



✅ Installed Node.js and npm  

✅ Installed Stellar SDK  

✅ Wrote create-account.js  

✅ Wrote send-payment.js  

✅ Wrote check-balance.js  

✅ All scripts worked  

✅ Installed Stellar CLI  

✅ Generated CLI identity  

✅ Funded CLI account  

✅ Deployed smart contract  

✅ Got Contract ID  

✅ Invoked hello function  

✅ Verified on StellarExpert



\*\*I wrote code that runs on a real blockchain. Less than 1% of people can do this.\*\*



---



\## Key Moment



The moment I ran my first script and saw it create an account automatically - that's when I realized I'm not just using blockchain anymore. I'm building on it.



---



\## Next Class: Rust from Zero



\*\*Tuesday October 14, 2025\*\*



What I'll learn:

\- 🦀 Basic Rust syntax

\- 📦 Variables, types, functions

\- 🎯 Ownership (Rust's superpower)

\- 📝 Write hello world in Rust



\*\*Why Rust?\*\*

\- It's Soroban's language

\- Eliminates bugs other languages allow

\- No null pointers, no memory leaks, no race conditions

\- If it compiles, it probably works

\- Rust developers are among highest paid



\*\*Preparation:\*\*

Install Rust before next class:

```bash

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustc --version

cargo --version

```



---



\## Reflection



\*\*Hardest moment:\*\* Understanding async/await at first  

\*\*WOW moment:\*\* Seeing my contract live on blockchain  

\*\*Most useful script:\*\* create-account.js (automates everything)  

\*\*What I'd build:\*\* Payment system for family business



The code I wrote today is permanent. It's on blockchain. Nobody can erase it. Nobody can take away what I learned.



---



\*\*Status:\*\* Blockchain developer (not just user anymore) 🚀

