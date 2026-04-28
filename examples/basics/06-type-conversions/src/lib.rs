#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, Env, String, TryFromVal, Val};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ConversionError {
    InvalidString = 1,
}

#[contract]
pub struct ConversionContract;

#[contractimpl]
impl ConversionContract {
    pub fn try_convert(env: Env, val: Val) -> Result<String, ConversionError> {
        // Try to convert val into a String, map error if failed
        String::try_from_val(&env, &val).map_err(|_| ConversionError::InvalidString)
    }
}
