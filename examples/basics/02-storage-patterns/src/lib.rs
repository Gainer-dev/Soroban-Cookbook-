//! # Storage Patterns Contract
//!
//! Demonstrates the three types of storage available in Soroban:
//! - Persistent: Data that lives permanently (requires TTL management)
//! - Temporary: Data that only exists for the current ledger
//! - Instance: Data tied to the contract instance lifetime
//!
//! Each storage type has different cost and lifetime characteristics.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Persistent(Symbol),
    Temporary(Symbol),
    Instance(Symbol),
}

/// Storage contract demonstrating all three storage types
#[contract]
pub struct StorageContract;

#[contractimpl]
impl StorageContract {
    // ==================== PERSISTENT STORAGE ====================

    /// Stores a value in persistent storage and extends its TTL.
    pub fn set_persistent(env: Env, key: Symbol, value: u64) {
        let storage_key = DataKey::Persistent(key.clone());
        env.storage().persistent().set(&storage_key, &value);
        env.storage().persistent().extend_ttl(&storage_key, 100, 100);
        env.events().publish((symbol_short!("persist"), symbol_short!("set")), (key, value));
    }

    /// Retrieves a value from persistent storage.
    /// Returns `Some(value)` if present, or `None`.
    pub fn get_persistent(env: Env, key: Symbol) -> Option<u64> {
        env.storage().persistent().get(&DataKey::Persistent(key))
    }

    /// Checks if a key exists in persistent storage.
    pub fn has_persistent(env: Env, key: Symbol) -> bool {
        env.storage().persistent().has(&DataKey::Persistent(key))
    }

    /// Removes a value from persistent storage and emits an event.
    pub fn remove_persistent(env: Env, key: Symbol) {
        env.storage().persistent().remove(&DataKey::Persistent(key.clone()));
        env.events()
            .publish((symbol_short!("persist"), symbol_short!("remove")), key);
    }

    // ==================== TEMPORARY STORAGE ====================

    /// Stores a value in temporary storage.
    /// Temporary data only exists for the current ledger - cheapest option.
    ///
    /// # Arguments
    /// * `key` - The storage key
    /// * `value` - The value to store
    ///
    /// # Cost
    /// Lowest cost, no rent required
    ///
    /// # Use Cases
    /// - Intermediate calculations
    /// - Transaction-scoped flags
    /// - Temporary state within a single operation
    pub fn set_temporary(env: Env, key: Symbol, value: u64) {
        env.storage().temporary().set(&DataKey::Temporary(key.clone()), &value);
        env.events()
            .publish((symbol_short!("temp"), symbol_short!("set")), (key, value));
    }

    /// Retrieves a value from temporary storage.
    /// Returns `Some(value)` if present, or `None`.
    pub fn get_temporary(env: Env, key: Symbol) -> Option<u64> {
        env.storage().temporary().get(&DataKey::Temporary(key))
    }

    /// Checks if a key exists in temporary storage.
    pub fn has_temporary(env: Env, key: Symbol) -> bool {
        env.storage().temporary().has(&DataKey::Temporary(key))
    }

    // ==================== INSTANCE STORAGE ====================

    /// Stores a value in instance storage.
    /// Instance data lives as long as the contract instance exists.
    ///
    /// # Arguments
    /// * `key` - The storage key
    /// * `value` - The value to store
    ///
    /// # Cost
    /// Medium cost, requires rent (but cheaper than persistent)
    ///
    /// # Use Cases
    /// - Contract configuration
    /// - Admin addresses
    /// - Contract metadata
    pub fn set_instance(env: Env, key: Symbol, value: u64) {
        env.storage().instance().set(&DataKey::Instance(key), &value);

        // Extend instance storage TTL
        env.storage().instance().extend_ttl(100, 100);

        // EVENT: Instance storage updated
        env.events().publish(
            (symbol_short!("instance"), symbol_short!("set")),
            (key, value),
        );
    }

    /// Retrieves a value from instance storage.
    ///
    /// # Returns
<<<<<<< HEAD
    /// `Some(value)` if the key exists, `None` if it doesn't.
    pub fn get_instance(env: Env, key: Symbol) -> Option<u64> {
        env.storage().instance().get(&key)
=======
    /// The stored value, or panics if key doesn't exist
    pub fn get_instance(env: Env, key: Symbol) -> u64 {
        env.storage().instance().get(&DataKey::Instance(key)).unwrap()
>>>>>>> b9af063 (Storage Patterns)
    }

    /// Checks if a key exists in instance storage.
    pub fn has_instance(env: Env, key: Symbol) -> bool {
        env.storage().instance().has(&DataKey::Instance(key))
    }

    /// Removes a value from instance storage.
    pub fn remove_instance(env: Env, key: Symbol) {
        env.storage().instance().remove(&DataKey::Instance(key.clone()));
        env.events()
            .publish((symbol_short!("instance"), symbol_short!("remove")), key);
    }
}

#[cfg(test)]
mod test;
