# Class 4 – Error Handling and Storage in Soroban

This class was all about making contracts safer, cleaner, and ready for real use on the blockchain.  
It focused on two big ideas: how to handle errors correctly, and how to store data efficiently.

---

## Error Handling

In blockchain, a panic doesn’t just break the code — it also **wastes gas**.  
That’s why every action should fail gracefully and predictably.

- **Option<T>** → for values that might not exist yet (for example, a new user’s balance).  
- **Result<T, E>** → for operations that can fail and return a specific error.

We learned to define our own errors with `#[contracterror]`, making debugging and gas usage easier.  
Example:  
`BalanceInsufficient`, `InvalidAmount`, `NotAuthorized`, `NotInitialized`.

### Validation order
1. Authentication → `require_auth()`  
2. Cheap input checks → `amount > 0`  
3. Read from storage  
4. Validate the state (enough balance, permissions)  
5. Modify storage only if everything passed

The `?` operator simplifies the code and stops execution automatically when something fails.  
`ok_or()` converts an `Option` into a `Result` so missing data can become a proper error.

**Key idea:** *Fail fast, save gas.*

---

## Storage in Soroban

Storage on-chain is expensive — you pay for every byte — so it’s important to use the right type:

- **Instance storage:** global configuration (e.g., admin, total counters)  
- **Persistent storage:** critical per-user data (balances, ownership)  
- **Temporary storage:** short-term cache or data that can expire (prices, locks)

We now organize all keys using a `DataKey` enum instead of plain strings:

```rust
pub enum DataKey {
  Admin,
  Balance(Address),
  Donation(u32),
}
```

This keeps everything type-safe, avoids typos, and makes the contract easier to maintain.

---

## Time To Live (TTL)

Every stored item has an expiration time.  
To keep important data alive, we extend its TTL periodically:

```rust
extend_ttl(threshold, extend_to)
```

Typical values: `100, 100`.  
Always extend **after** a successful operation.

---

## Best Practices

- Use `has()` when only checking if a value exists.  
- Keep instance data for global state, persistent for users, temporary for cache.  
- Avoid literal strings for keys — use enums.  
- Extend TTLs in critical paths.  
- Validate before reading or writing storage.

In short: be efficient, structured, and predictable.
