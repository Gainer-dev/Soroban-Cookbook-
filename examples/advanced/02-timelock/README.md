# Timelock

A time-delayed execution pattern for Soroban smart contracts. Operations are queued with a mandatory waiting period before they can be executed, providing a safety window for review or cancellation.

## 📖 What You'll Learn

- How to implement a queue → wait → execute workflow on-chain
- Using persistent storage to track scheduled operations
- Enforcing minimum and maximum delays with ledger timestamps
- Emitting structured events for each lifecycle stage

## 🎯 Overview

The timelock pattern is a core building block for governance and security-critical contracts. It prevents instant execution of sensitive actions by requiring a delay between scheduling and execution.

```
Admin queues operation  →  delay passes  →  Admin executes
         ↓                                        ↑
    Admin can cancel ──────────────────────────────
```

**Constants:**
- `MIN_DELAY`: 60 seconds
- `MAX_DELAY`: 86,400 seconds (24 hours)

## 🔑 Key Concepts

### Queue an Operation

```rust
pub fn queue(env: Env, operation_id: Bytes, delay: u64) {
    // delay must be within MIN_DELAY..=MAX_DELAY
    let execute_at = env.ledger().timestamp() + delay;
    env.storage().persistent().set(&DataKey::Operation(operation_id), &execute_at);
}
```

### Execute After Delay

```rust
pub fn execute(env: Env, operation_id: Bytes) {
    let execute_at: u64 = env.storage().persistent().get(&key).expect("not found");
    if env.ledger().timestamp() < execute_at {
        panic!("Too early");
    }
    env.storage().persistent().remove(&key); // prevents replay
}
```

### Cancel Before Execution

```rust
pub fn cancel(env: Env, operation_id: Bytes) {
    env.storage().persistent().remove(&DataKey::Operation(operation_id));
}
```

### Operation States

| State     | Meaning                                      |
|-----------|----------------------------------------------|
| `Unknown` | Not queued                                   |
| `Pending` | Queued, delay not yet passed                 |
| `Ready`   | Delay passed, can be executed                |
| `Done`    | Executed and removed from storage            |

## 🔒 Security Notes

- Only the admin can queue, execute, or cancel operations
- Operations are removed after execution to prevent replay attacks
- TTL is extended to `120,960` ledgers (~7 days) to ensure operations don't expire before `MAX_DELAY` passes

## 🚀 Running Tests

```bash
cargo test -p timelock
```

## 📚 Related Examples

- [01-multi-party-auth](../01-multi-party-auth/) — Multi-party authorization patterns
- [Governance Examples](../../governance/) — DAOs and voting systems that use timelocks
- [Advanced Examples](../) — Other complex patterns
