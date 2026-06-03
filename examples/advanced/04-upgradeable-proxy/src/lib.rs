//! # Proxy Pattern Example
//!
//! This example demonstrates an upgradeable proxy pattern that separates
//! the proxy and implementation contracts. The proxy forwards calls to an
//! implementation contract, allowing safe upgrades while preserving storage.

#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, vec, Address, Env, Symbol};

const ADMIN_KEY: Symbol = symbol_short!("admin");
const IMPLEMENTATION_KEY: Symbol = symbol_short!("impl");

/// The proxy contract that forwards calls to the implementation.
/// This contract handles upgrades and maintains the admin control.
#[contract]
pub struct ProxyContract;

#[contractimpl]
impl ProxyContract {
    /// Initialize the proxy with an admin and initial implementation.
    ///
    /// # Arguments
    /// * `env` - the execution environment
    /// * `admin` - the address that can authorize upgrades
    /// * `implementation` - the initial implementation contract address
    pub fn init(env: Env, admin: Address, implementation: Address) {
        if env.storage().persistent().has(&ADMIN_KEY) {
            panic!("Already initialized");
        }

        env.storage().persistent().set(&ADMIN_KEY, &admin);
        env.storage()
            .persistent()
            .set(&IMPLEMENTATION_KEY, &implementation);
    }

    /// Upgrade to a new implementation contract.
    /// Only the admin can call this.
    ///
    /// # Arguments
    /// * `env` - the execution environment
    /// * `new_implementation` - the address of the new implementation contract
    pub fn upgrade(env: Env, new_implementation: Address) {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&ADMIN_KEY)
            .unwrap_or_else(|| panic!("Admin not set"));

        admin.require_auth();

        env.storage()
            .persistent()
            .set(&IMPLEMENTATION_KEY, &new_implementation);

        env.events()
            .publish((symbol_short!("upgraded"),), new_implementation);
    }

    /// Get the current implementation address.
    ///
    /// # Returns
    /// The address of the current implementation contract
    pub fn get_implementation(env: Env) -> Address {
        env.storage()
            .persistent()
            .get(&IMPLEMENTATION_KEY)
            .unwrap_or_else(|| panic!("Implementation not set"))
    }

    /// Forward a call to the implementation contract's add function.
    ///
    /// # Arguments
    /// * `env` - the execution environment
    /// * `a` - first number to add
    /// * `b` - second number to add
    ///
    /// # Returns
    /// The sum of a and b
    pub fn add(env: Env, a: i128, b: i128) -> i128 {
        let implementation: Address = Self::get_implementation(&env);

        env.invoke_contract(&implementation, &symbol_short!("add"), vec![&env, a, b])
    }

    /// Forward a call to the implementation contract's subtract function.
    ///
    /// # Arguments
    /// * `env` - the execution environment
    /// * `a` - the minuend
    /// * `b` - the subtrahend
    ///
    /// # Returns
    /// The difference (a - b)
    pub fn subtract(env: Env, a: i128, b: i128) -> i128 {
        let implementation: Address = Self::get_implementation(&env);

        env.invoke_contract(&implementation, &symbol_short!("sub"), vec![&env, a, b])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_proxy_initialization() {
        let env = Env::default();
        let admin = soroban_sdk::Address::random(&env);
        let implementation = soroban_sdk::Address::random(&env);

        ProxyContract::init(&env, admin.clone(), implementation.clone());
        assert_eq!(ProxyContract::get_implementation(&env), implementation);
    }

    #[test]
    #[should_panic(expected = "Already initialized")]
    fn test_double_initialization_fails() {
        let env = Env::default();
        let admin = soroban_sdk::Address::random(&env);
        let implementation = soroban_sdk::Address::random(&env);

        ProxyContract::init(&env, admin.clone(), implementation.clone());
        ProxyContract::init(&env, admin.clone(), implementation.clone());
    }

    #[test]
    fn test_upgrade_only_by_admin() {
        let env = Env::default();
        let admin = soroban_sdk::Address::random(&env);
        let impl_v1 = soroban_sdk::Address::random(&env);
        let impl_v2 = soroban_sdk::Address::random(&env);

        ProxyContract::init(&env, admin.clone(), impl_v1.clone());
        assert_eq!(ProxyContract::get_implementation(&env), impl_v1);

        // Admin upgrades the implementation
        admin.require_auth();
        ProxyContract::upgrade(&env, impl_v2.clone());
        assert_eq!(ProxyContract::get_implementation(&env), impl_v2);
    }
}
