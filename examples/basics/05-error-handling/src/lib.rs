//! # Panic vs. Errors in Soroban
//!
//! Soroban contracts have two failure modes: **panics** (unrecoverable aborts)
//! and **errors** (typed, recoverable values the caller can inspect).
//!
//! ## Decision Rule
//!
//! | Situation | Mechanism | Why |
//! |-----------|-----------|-----|
//! | Invariant that should never be false | `panic!` / `panic_with_error!` | Signals a bug; no recovery makes sense |
//! | Auth failure (`require_auth`) | Soroban panics internally | Unauthorized callers must be rejected hard |
//! | Expected bad input from caller | `Err(ContractError::…)` | Caller can handle and retry |
//! | Business-logic constraint violated | `Err(ContractError::…)` | Predictable; documentable; testable |
//! | Reached truly impossible branch | `panic!("unreachable: …")` | Defensive; keeps the type system happy |
//!
//! ## Performance Note
//!
//! Both aborts consume the submitted fee; there is no gas "refund" for a
//! cleaner error path. Prefer typed errors for *user-facing* failures because
//! they allow the client to react without re-submitting a doomed transaction.
//!
//! ## Anatomy
//!
//! ```text
//! ┌────────────────────┬────────────────────────────────────────────┐
//! │  Panic             │  Typed Error                               │
//! ├────────────────────┼────────────────────────────────────────────┤
//! │  panic!("msg")     │  #[contracterror] enum + Result<T, E>      │
//! │  panic_with_error! │  ? operator / map_err                      │
//! │  Immediate abort   │  Caller sees u32 discriminant via XDR      │
//! │  No info to caller │  Documentable, testable variants           │
//! └────────────────────┴────────────────────────────────────────────┘
//! ```
#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, symbol_short, Address,
    Env, Symbol,
};

// ---------------------------------------------------------------------------
// Error enum
// ---------------------------------------------------------------------------

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    ZeroAmount = 100,
    InsufficientBalance = 101,
    InputTooLong = 102,
    ContractPaused = 200,
    Unauthorized = 201,
    Overflow = 202,
}

// ---------------------------------------------------------------------------
// Storage key type
// ---------------------------------------------------------------------------

#[contracttype]
pub enum DataKey {
    Balance(Address),
    Admin,
    Paused,
}

// ---------------------------------------------------------------------------
// Audit event payload
// ---------------------------------------------------------------------------

#[contracttype]
pub struct LedgerEventData {
    pub amount: i128,
    pub action: Symbol,
}

#[contract]
pub struct ErrorDemoContract;

#[contractimpl]
impl ErrorDemoContract {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("contract already initialised");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Paused, &false);
    }

    pub fn deposit(env: Env, from: Address, amount: i128) -> Result<i128, ContractError> {
        if amount == 0 {
        pub fn withdraw(env: Env, from: Address, amount: i128) -> Result<i128, ContractError> {
            if amount == 0 {
                return Err(ContractError::ZeroAmount);
            }

            let paused: bool = env.storage().instance().get(&DataKey::Paused).unwrap_or(false);
            if paused {
                return Err(ContractError::ContractPaused);
            }

            from.require_auth();

            let key = DataKey::Balance(from.clone());
            let balance: i128 = env.storage().persistent().get(&key).unwrap_or(0);

            if balance < amount {
                return Err(ContractError::InsufficientBalance);
            }

            let new_balance = balance - amount; // safe: we checked above
            env.storage().persistent().set(&key, &new_balance);

            env.events().publish(
                (symbol_short!("errdemo"), symbol_short!("withdraw"), from.clone()),
                LedgerEventData {
                    amount,
                    action: symbol_short!("withdraw"),
                },
            );

            Ok(new_balance)
        }

        pub fn pause(env: Env, caller: Address) {
            caller.require_auth();

            let admin: Address = env
                .storage()
                .instance()
                .get(&DataKey::Admin)
                .unwrap_or_else(|| panic!("uninitialised: admin key missing"));

            if caller != admin {
                panic_with_error!(env, ContractError::Unauthorized);
            }

            env.storage().instance().set(&DataKey::Paused, &true);
        }

        pub fn unpause(env: Env, caller: Address) {
            caller.require_auth();

            let admin: Address = env
                .storage()
                .instance()
                .get(&DataKey::Admin)
                .unwrap_or_else(|| panic!("uninitialised: admin key missing"));

            if caller != admin {
                panic_with_error!(env, ContractError::Unauthorized);
            }

            env.storage().instance().set(&DataKey::Paused, &false);
        }
    pub fn withdraw(env: Env, from: Address, amount: i128) -> Result<i128, ContractError> {
        if amount == 0 {
            return Err(ContractError::ZeroAmount);
        }

        let paused: bool = env
            .storage()
            .instance()
            .get(&DataKey::Paused)
            .unwrap_or(false);
        if paused {
            return Err(ContractError::ContractPaused);
        }

        from.require_auth();

        let key = DataKey::Balance(from.clone());
        let balance: i128 = env.storage().persistent().get(&key).unwrap_or(0);

        if balance < amount {
            return Err(ContractError::InsufficientBalance);
        }

        let new_balance = balance - amount; // safe: we checked above
        env.storage().persistent().set(&key, &new_balance);

        env.events().publish(
            (symbol_short!("errdemo"), symbol_short!("withdraw"), from),
            LedgerEventData {
                amount,
                action: symbol_short!("withdraw"),
            },
        );

        Ok(new_balance)
    }

    // =======================================================================
    // Example B — panic_with_error! for hard invariant violations
    // =======================================================================

    /// Pause the contract.  Only the registered admin may do this.
    ///
    /// ### Why `panic_with_error!` instead of `Err(Unauthorized)`?
    ///
    /// This is an *administrative invariant*: if your code allows a non-admin
    /// to reach this branch, you have a bug in your auth logic, not a user
    /// input error.  `panic_with_error!` lets you attach the error code for
    /// off-chain diagnostics while still aborting the transaction hard.
    ///
    /// Compare: returning `Err(Unauthorized)` would imply the caller could
    /// "try again differently" — but there is nothing to retry.
    pub fn pause(env: Env, caller: Address) {
        caller.require_auth();

        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            // Panic: if Admin key is missing the contract was never initialised.
            // This should be impossible after `initialize` runs; treat as a bug.
            .unwrap_or_else(|| panic!("uninitialised: admin key missing"));

        if caller != admin {
            // Hard abort with a typed code.  Off-chain tools can read
            // `ContractError::Unauthorized` (discriminant 201) from the
            // transaction result even though the transaction reverted.
            panic_with_error!(env, ContractError::Unauthorized);
        }

        env.storage().instance().set(&DataKey::Paused, &true);
    }

<<<<<<< HEAD
    /// ✅ GOOD: Result for business logic errors
    /// Division by zero is expected user error, not a bug
    pub fn divide(a: i128, b: i128) -> Result<i128, Error> {
        Ok(Self::divide_checked(a, b).map_err(Error::from)?)
    }

    /// Core division operation returning a domain-specific error type.
    fn divide_checked(a: i128, b: i128) -> Result<i128, MathError> {
        if b == 0 {
            return Err(MathError::DivisionByZero);
=======
    /// Unpause the contract (admin only).
    pub fn unpause(env: Env, caller: Address) {
        caller.require_auth();

        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| panic!("uninitialised: admin key missing"));

        if caller != admin {
            panic_with_error!(env, ContractError::Unauthorized);
>>>>>>> f5b9735 (feat(examples): panic vs errors demo – issue #260)
        }

        env.storage().instance().set(&DataKey::Paused, &false);
    }

    pub fn status_label(_env: Env, code: u32) -> Symbol {
        match code {
            0 => symbol_short!("ok"),
            1 => symbol_short!("paused"),
            2 => symbol_short!("error"),
            _ => panic!("unknown status code: this is a bug"),
        }
    }

    pub fn balance(env: Env, account: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(account))
            .unwrap_or(0)
    }

    pub fn is_paused(env: Env) -> bool {
        env.storage().instance().get(&DataKey::Paused).unwrap_or(false)
    }
}

#[cfg(test)]
mod test;