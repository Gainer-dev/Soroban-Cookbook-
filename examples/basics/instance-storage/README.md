# Instance Storage

A focused demonstration of Soroban's instance storage — the middle tier between persistent and temporary storage.

## 📖 What You'll Learn

- When and why to use instance storage over persistent or temporary storage
- How all instance keys share a single TTL (unlike persistent's per-key TTL)
- Two real-world use cases: transaction counters and runtime configuration
- How to extend instance TTL efficiently with a single call

## 🎯 Overview

Instance storage is scoped to the deployed contract address. All keys share one TTL, so a single `extend_ttl` call protects every key at once — making it cheaper to manage than persistent storage for contract-wide state.

| Property                  | Persistent     | Instance       | Temporary      |
|---------------------------|----------------|----------------|----------------|
| Survives contract upgrade | ✅ Yes         | ❌ No          | ❌ No          |
| TTL management            | Per-key        | Per-instance   | Per-key        |
| Relative cost             | Highest        | Medium         | Lowest         |
| Best for                  | User balances  | Contract config| Short-lived    |

## 🔑 Key Concepts

### Shared TTL

```rust
// One call refreshes ALL instance keys — no per-key bookkeeping needed
env.storage().instance().extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);
```

Compare this to persistent storage, where you must call `extend_ttl` once per key.

### Use Case 1 — Transaction Counter

```rust
pub fn increment_counter(env: Env) -> u64 {
    let count: u64 = env.storage().instance()
        .get(&InstanceKey::TxCounter)
        .unwrap_or(0) + 1;
    env.storage().instance().set(&InstanceKey::TxCounter, &count);
    env.storage().instance().extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);
    count
}
```

### Use Case 2 — Runtime Configuration

```rust
pub fn set_config(env: Env, key: Symbol, value: u64) {
    env.storage().instance().set(&InstanceKey::Config(key), &value);
    env.storage().instance().extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);
}
```

## ✅ When to Use Instance Storage

**Use instance storage when:**
- Data is contract-wide (not per-user)
- Data does not need to survive a contract upgrade
- You want simpler TTL management than persistent storage

**Don't use instance storage when:**
- Data must survive `upgrade()` calls → use persistent
- Data is only needed for one invocation → use temporary
- Data is per-user (e.g. balances) → use persistent

## 🚀 Running Tests

```bash
cargo test -p instance-storage
```

## 📚 Related Examples

- [persistent-storage](../persistent-storage/) — Per-key TTL, user balances
- [temporary_storage](../temporary_storage/) — Single-ledger caches
- [02-storage-patterns](../02-storage-patterns/) — All three types compared
