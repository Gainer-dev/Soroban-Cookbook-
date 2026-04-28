#![cfg(test)]
use super::*;
use soroban_sdk::{Env, TryFromVal};

#[test]
fn test_conversion_failure() {
    let env = Env::default();
    let val = 123u32.into_val(&env);
    // Should fail context try_from as string
    let _err = <soroban_sdk::String as TryFromVal<Env, soroban_sdk::Val>>::try_from_val(&env, &val);
}
