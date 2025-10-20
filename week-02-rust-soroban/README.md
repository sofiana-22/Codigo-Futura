<p align="center">
  <img src="./assets/tiburona.png" alt="Tiburona - Buen Día Builders" width="380"/>
</p>

<h2 align="center">🦈 Week 2 — Rust & Soroban</h2>
<p align="center">Learning Rust fundamentals and deploying smart contracts with Soroban CLI.</p>


# Week 2 – From Hello World to Real Contracts

This week felt like the point where everything started to make sense.  
We went beyond “Hello World” and learned what it takes to build professional Soroban contracts.

---

## What We Did

- Revisited the **Hello World** project and deployed it to the Stellar Testnet.  
- Understood the project structure (`Cargo.toml`, `lib.rs`, `Makefile`) and how the build process works.  
- Learned how to handle **errors safely** using `Option`, `Result`, and custom enums.  
- Practiced secure validation using `require_auth()` and the `?` operator.  
- Explored how Soroban stores data and how to choose between:
  - `instance` for global configuration  
  - `persistent` for important user data  
  - `temporary` for cache or quick data  
- Discovered how **TTL (Time To Live)** prevents data from expiring unexpectedly.  
- Applied everything to design safer, cleaner, and more scalable contracts.

---

## Key Takeaways

- Validate before touching storage.  
- Always use structured `DataKey` enums.  
- Handle every possible error explicitly.  
- Keep storage lightweight and organized.  
- Extend TTLs when data matters.  

By the end of Week 2, I can deploy a contract that:
- Handles errors correctly,  
- Manages storage efficiently, and  
- Behaves predictably on the blockchain.  

Next up: refining the **“Hello Tiburona”** contract and integrating all these best practices into one final version.
