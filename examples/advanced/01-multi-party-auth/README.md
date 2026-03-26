1# Multi-Party Authorization Patterns

This example demonstrates advanced multi-party authorization in Soroban,
including N-of-N multi-sig, M-of-N threshold approval, sequential escrow,
and a compact **authorization vector** format for serializing, validating,
and storing signer sets on-chain.

---

## Authorization Vector Format

An *auth vector* is a length-prefixed, sorted, deduplicated list of signer
addresses serialized into a `Bytes` blob. It is designed for compact on-chain
storage and safe cross-contract passing.

### Wire format

```
[ count: u32 (4 bytes, big-endian) ]
[ addr_0: 56 bytes (ASCII strkey)  ]
[ addr_1: 56 bytes                 ]
...
[ addr_{count-1}: 56 bytes         ]
```

Total blob size: `4 + count × 56` bytes.

Each address is stored as its 56-character ASCII strkey — `G…` for user
accounts, `C…` for contract addresses — which is the canonical Stellar
encoding and is directly accepted by `Address::from_string_bytes`.

### Constraints

| # | Constraint | Enforced by |
|---|-----------|-------------|
| 1 | `count` matches the number of address entries in the blob | `decode_auth_vec`, `validate_auth_vec` |
| 2 | Addresses are in **strict ascending** lexicographic order of their strkey bytes | `decode_auth_vec`, `validate_auth_vec` |
| 3 | **No duplicate** addresses (strict ordering implies uniqueness) | same as above |
| 4 | `1 ≤ count ≤ MAX_SIGNERS` (currently 20) | `encode_auth_vec`, `decode_auth_vec` |

Because encoding is canonical, two blobs representing the same signer set
are byte-for-byte identical. This makes equality checks and content-addressed
storage trivial.

### Constants

| Constant | Value | Meaning |
|----------|-------|---------|
| `MAX_SIGNERS` | 20 | Maximum addresses per vector |
| `ADDR_BYTES` | 56 | Bytes per address entry |
| `HEADER_LEN` | 4 | Bytes for the count header |

---

## API Reference

### `encode_auth_vec(env, signers) → Bytes`

Encodes a `Vec<Address>` into a canonical auth-vector blob. The input is
**sorted and deduplicated** before encoding, so the output is identical
regardless of the order addresses are supplied.

Panics if the list is empty or contains more than `MAX_SIGNERS` unique
addresses after deduplication.

```rust
let signers = Vec::from_array(&env, [alice.clone(), bob.clone(), carol.clone()]);
let blob: Bytes = client.encode_auth_vec(&signers);
// blob is 4 + 3×56 = 172 bytes, addresses in ascending strkey order
```

### `decode_auth_vec(env, encoded) → Vec<Address>`

Decodes a blob back into a `Vec<Address>`. Validates all constraints before
returning — panics on any violation so callers never receive a malformed
vector.

```rust
let signers: Vec<Address> = client.decode_auth_vec(&blob);
```

### `validate_auth_vec(env, encoded) → bool`

Cheap pre-flight check. Returns `true` if the blob is well-formed, `false`
otherwise. Does **not** panic — use this before passing an untrusted blob to
`decode_auth_vec` or `multi_sig_transfer_encoded`.

```rust
if !client.validate_auth_vec(&untrusted_blob) {
    // reject early
}
```

### `auth_vec_len(env, encoded) → u32`

Returns the signer count from the header without decoding addresses.

```rust
let n: u32 = client.auth_vec_len(&blob); // e.g. 3
```

### `auth_vec_contains(env, encoded, signer) → bool`

Returns `true` if `signer` is present in the encoded vector.

```rust
let present: bool = client.auth_vec_contains(&blob, &alice);
```

### `multi_sig_transfer_encoded(env, encoded_signers, to, amount)`

Variant of `multi_sig_transfer` that accepts a pre-encoded blob. Decodes and
validates the vector, then calls `require_auth()` on every signer. Useful
when the same signer set is stored on-chain and reused across many calls.

```rust
client.multi_sig_transfer_encoded(&blob, &recipient, &500i128);
```

---

## Patterns Demonstrated

### 1. N-of-N Multi-Sig (`multi_sig_transfer`)

Every address in the list must authorize. The Soroban host collects and
verifies all signatures atomically — order of `require_auth()` calls does
not matter.

```rust
pub fn multi_sig_transfer(_env: Env, signers: Vec<Address>, _to: Address, _amount: i128) {
    for signer in signers.iter() {
        signer.require_auth();
    }
}
```

**When to use:** Joint custody, mandatory all-party approval, treasury
operations where every key-holder must consent.

**Gas note:** Scales linearly with signer count. Bound the list size in
production to prevent unbounded-loop attacks.

### 2. M-of-N Threshold (`proposal_approval`)

At least `threshold` parties from a stored valid-signers list must authorize.
The valid-signers set is checked on every call to prevent unauthorized
addresses from contributing to the threshold.

```rust
// Setup: 2-of-3 multisig
client.setup_proposal(&proposal_id, &2u32, &all_signers);

// Execution: any 2 of the 3 approve
let approvers = Vec::from_array(&env, [alice.clone(), carol.clone()]);
client.proposal_approval(&proposal_id, &approvers);
```

**When to use:** DAO governance, shared wallets, protocol upgrades where a
supermajority (not unanimity) is required.

### 3. Sequential Escrow (`sequential_auth_escrow`)

A two-step workflow where different parties authorize different steps.

- **Step 0 → 2:** Buyer funds the escrow (buyer's auth only).
- **Step 2 → 0:** Both buyer and seller jointly release (2-of-2).

```rust
// Step 1: buyer funds
client.sequential_auth_escrow(&buyer, &seller, &1000i128);

// Step 2: joint release
client.sequential_auth_escrow(&buyer, &seller, &1000i128);
```

**When to use:** Escrow services, multi-stage workflows where authorization
requirements change as the process advances.

---

## Security Considerations

**Validate before decode.** Call `validate_auth_vec` on any blob received
from an untrusted source before passing it to `decode_auth_vec` or
`multi_sig_transfer_encoded`.

**Bound signer lists.** `MAX_SIGNERS = 20` prevents unbounded loops. Adjust
for your use case but always enforce a cap.

## Usage Tip

When integrating these patterns into your own contracts, always validate
signer lists and thresholds at contract initialization to avoid accidental
misconfiguration or security gaps. Consider providing admin functions to
update signers or thresholds securely. The provided examples focus on
authorization logic and do not perform actual token transfers.

## How to run tests

```bash
cargo test -p multi-party-auth
```

Tests cover the encode/decode/validation helpers, auth enforcement paths
and multi-step escrow workflows. See the `tests/` module for full coverage.

## Building

```bash
cargo build -p multi-party-auth --target wasm32-unknown-unknown --release
```

## Related Examples

- [03-authentication](../../basics/03-authentication/) — single-party auth patterns
- [05-auth-context](../../basics/05-auth-context/) — cross-contract auth context
- [intermediate/multi-sig-patterns](../../intermediate/multi-sig-patterns/) — on-chain multi-sig wallet

## Usage Tip

When integrating these patterns into your own contracts, always validate
signer lists and thresholds at contract initialization to avoid accidental
misconfiguration or security gaps. Consider providing admin functions to
update signers or thresholds securely. The provided examples focus on
authorization logic and do not perform actual token transfers.
