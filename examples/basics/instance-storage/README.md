# Instance Storage

A focused deep dive into Soroban's instance storage tier — the contract-wide, shared-TTL storage layer.

## Overview

Instance storage is scoped to the deployed contract address. All keys share a single TTL, so one `extend_ttl` call refreshes the lifetime of every key at once. This makes it cheaper to manage than persistent storage for contract-wide data.

## When to Use Instance Storage

| Use instance storage when… | Avoid it when… |
|---|---|
| Data is contract-wide config shared by all callers | Data is per-user or per-entity (use persistent) |
| You want simpler TTL management (one call covers all keys) | Data must survive a contract upgrade (use persistent) |
| Data changes moderately often | Data is only needed for one invocation (use temporary) |

**Typical use cases:** admin address, fee rates, protocol parameters, transaction counters, feature flags.

## Storage Comparison

| Property | Persistent | Instance | Temporary |
|---|---|---|---|
| Survives upgrade | ✅ Yes | ❌ No | ❌ No |
| TTL management | Per-key | Per-instance | Per-key |
| Relative cost | Highest | Medium | Lowest |
| Best for | Balances, ownership | Config, counters | Flags, caches |

## Key Concepts

### Shared TTL

Unlike persistent storage where each key has its own TTL, all instance keys share one TTL. A single `extend_ttl` call covers everything:

```rust
// One call refreshes ALL instance keys — no per-key bookkeeping needed.
env.storage().instance().extend_ttl(threshold, extend_to);
```

### Typed Key Enum

```rust
#[contracttype]
#[derive(Clone)]
pub enum InstanceKey {
    TxCounter,
    Config(Symbol),
}
```

Using a typed enum prevents key collisions at compile time and keeps the key surface explicit.

### CRUD Operations

```rust
// Write
env.storage().instance().set(&InstanceKey::TxCounter, &count);
env.storage().instance().extend_ttl(1_000, 10_000);

// Read
let count: u64 = env.storage().instance()
    .get(&InstanceKey::TxCounter)
    .unwrap_or(0);

// Check existence
let exists: bool = env.storage().instance().has(&InstanceKey::TxCounter);

// Delete
env.storage().instance().remove(&InstanceKey::TxCounter);
```

## Contract API

| Function | Description |
|---|---|
| `set_instance(key, value)` | Store a `u64` under a named config key |
| `get_instance(key)` | Retrieve a `u64` by key, returns `None` if missing |
| `increment_counter()` | Increment the transaction counter, returns new value |
| `get_counter()` | Read the current transaction counter |
| `set_config(key, value)` | Store a named runtime configuration value |
| `get_config(key)` | Retrieve a named runtime configuration value |
| `extend_ttl()` | Explicitly bump the instance TTL |

## Build & Test

```bash
# From this directory
cargo test
cargo build --target wasm32-unknown-unknown --release

# From the repository root
cargo test -p instance-storage
cargo build -p instance-storage --target wasm32-unknown-unknown --release
```

## Related Examples

- [02-storage-patterns](../02-storage-patterns/) — Compare all three storage types side-by-side
- [persistent-storage](../persistent-storage/) — Per-key TTL for user-specific data
- [temporary_storage](../temporary_storage/) — Ephemeral, single-ledger data
