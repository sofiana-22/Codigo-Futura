# Class 3 — Rust Essentials for Soroban (My Notes)

> *Goal for myself:* understand **why** Rust makes Soroban contracts safe, not just how to type the code.

## 🧠 Big ideas I now understand
- **Gas & storage matter:** every byte on-chain costs money. Pick types that are lean.
- **Ownership prevents foot‑guns:** one owner per value; moves vs copies are explicit.
- **Borrowing = efficiency:** use `&T` to read, `&mut T` to modify without cloning.
- **Pattern matching forces completeness:** `match`, `Option`, and `Result` make me handle all cases.
- **Soroban’s `Env` is the gateway:** storage, events, cryptography, and heap allocation live behind `Env`.

---

## 🔢 Data types for contracts (what I’ll use and why)
- **`u128`** for token balances (fits anything; avoid overflow).
- **`u32`** for counters/IDs (cheap, good range).
- **`Symbol`** (≤10 ASCII chars, fixed cost) for keys/topics → **always** for storage keys and event names.
- **`String`** (variable, needs `&env`) for user text or long names.
- **`Vec<T>`** for dynamic lists (needs `&env` to create with `Vec::new(&env)`).

**Example (readable literal & Symbol key):**
```rust
let balance: u128 = 1_000_000_0000000; // 1,000,000.0000000
let key = symbol_short!("balance");     // compact identifier
```

**String vs Symbol (my rule of thumb):**
- Storage keys, event topics, enum-like states → **`Symbol`**
- User-provided or long/emoji text → **`String`** via `String::from_str(&env, "...")`

---

## 🔐 Ownership (why Rust is strict)
- One owner per value; **moves** transfer ownership (old binding becomes invalid).
- Small scalars (`u32`, `u128`, `bool`) implement **Copy**.
- Heap-backed types (`String`, `Vec`) **move** by default.

```rust
let s1 = String::from("hola");
let s2 = s1;       // MOVE, s1 invalid now
// println!("{}", s1); // ❌
println!("{}", s2);   // ✅
```

**What this buys me:** memory safety by construction (no dangling pointers, no double frees).

---

## 🔄 Borrowing (how to avoid waste)
- **Read-only:** `&T` (many at once).
- **Mutable:** `&mut T` (only **one** at a time).

```rust
fn length(s: &String) -> usize { s.len() }     // read
fn add_bang(s: &mut String) { s.push_str("!"); } // write
```

**Why it matters on-chain:** cloning large data costs gas; borrowing is cheaper and safer.

---

## 🎯 Pattern matching & safe error handling
- `match` forces me to cover every case.
- `Option<T>` replaces null (Some/None).
- `Result<T,E>` surfaces domain errors (Ok/Err).

```rust
let x: u8 = 255;
match x.checked_add(1) {
  Some(v) => v,
  None => panic!("overflow"),
}
```

**Production rule:** avoid `unwrap()` in contract logic; prefer `unwrap_or`, `ok_or`, or explicit `match`.

---

## 🧰 Soroban specifics I used today
- **`Env`** provides: `storage()`, `events()`, cryptography, and heap allocation hooks.
- `String::from_str(&env, "...")` and `Vec::new(&env)` require `&env`.
- Storage patterns via **instance storage**:
  - **Read → Validate → Modify → Save → Emit** (golden pattern).

**Counter read/increment/store/emit pattern:**
```rust
pub fn increment(env: Env) -> u32 {
    let mut count: u32 = env.storage()
        .instance()
        .get(&symbol_short!("COUNTER"))
        .unwrap_or(0);

    count = count.checked_add(1).expect("overflow");

    env.storage().instance().set(&symbol_short!("COUNTER"), &count);
    env.events().publish((symbol_short!("increment"),), count);
    count
}
```

---

## 📌 Full counter mental model (what happens in a tx)
1. **Read** current value from instance storage (`Option<u32>` → default 0).
2. **Validate** (e.g., prevent under/overflow).
3. **Modify** value in memory.
4. **Save** new value with `set(&key, &value)` (note the `&` borrows).
5. **Emit** event for transparency/UX.
6. **Return** result.

**Decrement gotcha:** guard `0 → 0 - 1` (underflow) and `panic!` early.

---

## 🧯 Common mistakes I’ll avoid
- Missing `mut` on variables I intend to change.
- Forgetting `&` before `symbol_short!(...)` in `get/set`.
- Using `String` as a key instead of `Symbol` → wastes storage.
- Multiple `&mut` borrows at the same time.
- `unwrap()` on storage reads in production (prefer defaults or `match`).

---

## 🧪 Testing mindset
- Use `Env::default()` and the auto‑generated `*Client` to call methods.
- Test both **happy path** and **failure** (`#[should_panic(expected = "...")]`).

```rust
#[test]
fn test_increment() {
  let env = Env::default();
  let id = env.register_contract(None, ContadorContract);
  let client = ContadorContractClient::new(&env, &id);
  assert_eq!(client.increment(), 1);
  assert_eq!(client.get_count(), 1);
}
```

---

## 🛠️ Quick CLI checks (so I don’t get stuck)
```bash
rustc --version     # >= 1.70
cargo --version     # >= 1.70
stellar --version   # >= 20.0.0
```

---

## ✅ What I can now explain (to future me)
- When to choose `u32` vs `u128` (range + gas tradeoff).
- Why `Symbol` beats `String` for keys (fixed cost, validated at compile time).
- The **one mutable borrow** rule and why it prevents data races.
- Why `Option` on storage reads is a feature (uninitialized → default safely).

---

## ❓Questions I still want to explore
- Clean patterns for surfacing domain errors with `Result<T, E>` (no panics).
- When to use **persistent** vs **instance** storage for real apps.
- Gas tradeoffs of `Vec<T>` operations on larger datasets.

---

## 🧭 Practice plan before Class 4
- [ ] Add `increment_by(amount)` using `checked_add` + event `(inc_by, (amount, new))`.
- [ ] Add limit guard (e.g., max 1000) and a test that expects panic at the limit.
- [ ] Try switching the counter to `u8` and observe 255 wrap risks in tests.
- [ ] Replace any `unwrap()` with `unwrap_or`/`ok_or`/`match` and re‑run tests.

---

## TL;DR (one line for my brain)
> **Think in patterns:** *Read → Validate → Modify → Save → Emit*, with **ownership/borrowing** and **type choices** doing the safety heavy‑lifting.
