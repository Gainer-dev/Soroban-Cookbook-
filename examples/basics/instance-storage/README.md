# Instance Storage Pattern

A focused demonstration of Soroban instance storage, the storage map attached
to a deployed contract instance.

## What You'll Learn

- How to read and write values with `env.storage().instance()`
- Why instance storage is useful for small contract-wide state
- How shared instance TTL differs from persistent storage's per-key TTL
- When to choose instance storage and when to use persistent storage instead
- How to test counters, runtime config, and TTL keep-alive behavior

## Overview

Instance storage is physically attached to the contract instance ledger entry.
It shares the contract instance TTL, so a live contract instance keeps its
instance storage live too.

This makes instance storage a good fit for small, bounded state that is useful
across most calls:

- Admin or owner address
- Protocol configuration, such as fee basis points or limits
- Contract metadata and version flags
- Counters or aggregate totals
- Token pair metadata for a pool contract

Do not use instance storage for unbounded per-user or per-entity data. Instance
data is loaded with the contract instance, so large instance storage can make
every invocation more expensive.

## Instance vs Persistent Storage

| Feature | Instance Storage | Persistent Storage |
| --- | --- | --- |
| SDK API | `env.storage().instance()` | `env.storage().persistent()` |
| TTL scope | Shared with the contract instance | Independent per key |
| Size model | Limited by instance ledger-entry size | Supports unbounded keys |
| Best for | Small shared config/state | User, asset, proposal, or record data |
| Invocation footprint | Loaded with the contract instance | Loaded only when accessed |
| Expiration behavior | Archived with the instance | Archived per entry |

Use instance storage when the data is small, shared, and has a known upper
bound. Use persistent storage when the data grows with users, tokens, proposals,
orders, or other entities.

## Contract API

This example exposes six functions:

| Function | Purpose |
| --- | --- |
| `set_instance(key, value)` | Store a `u64` under a named config key |
| `get_instance(key)` | Read a named config key as `Option<u64>` |
| `increment_counter()` | Increment a contract-wide transaction counter |
| `get_counter()` | Read the counter, defaulting to `0` |
| `set_config(key, value)` | Semantic wrapper for runtime config |
| `get_config(key)` | Read runtime config as `Option<u64>` |
| `extend_ttl()` | Explicitly refresh the instance TTL |

## Key Pattern

The contract uses a typed key enum instead of raw symbols:

```rust
#[contracttype]
#[derive(Clone)]
pub enum InstanceKey {
    TxCounter,
    Config(Symbol),
}
```

Typed keys make the storage layout explicit and reduce accidental collisions.

## Writing Instance Data

```rust
pub fn set_instance(env: Env, key: Symbol, value: u64) {
    let storage_key = InstanceKey::Config(key);
    env.storage().instance().set(&storage_key, &value);
    env.storage().instance().extend_ttl(1_000, 10_000);
}
```

The important difference from persistent storage is that the TTL extension does
not name a specific key. A single instance TTL extension applies to the contract
instance and all instance storage entries.

## Use Case: Runtime Configuration

Runtime config is one of the safest instance-storage use cases because it is
small and contract-wide:

```rust
client.set_config(&symbol_short!("fee_bps"), &30);
assert_eq!(client.get_config(&symbol_short!("fee_bps")), Some(30));
```

Examples include protocol fees, caps, cooldown durations, feature flags, and
small version markers.

## Use Case: Contract Counter

The example also includes a counter:

```rust
assert_eq!(client.increment_counter(), 1);
assert_eq!(client.increment_counter(), 2);
assert_eq!(client.get_counter(), 2);
```
