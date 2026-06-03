//! # Factory Templates Example
//!
//! This example demonstrates a versioned factory pattern in Soroban:
//! 1. Template contracts that can be deployed repeatedly.
//! 2. A factory that stores version metadata for each template.
//! 3. Parameter validation before deployment and initialization.
//!
//! This pattern is useful when one factory needs to create multiple contract
//! shapes without hardcoding a single deployment path.

#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Bytes, BytesN, Env,
    Symbol, Vec,
};

pub const TEMPLATE_AJO: Symbol = symbol_short!("ajo");
pub const TEMPLATE_SAVINGS: Symbol = symbol_short!("savings");
pub const TEMPLATE_ESCROW: Symbol = symbol_short!("escrow");
pub const DEFAULT_VERSION: Symbol = symbol_short!("v1");

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum FactoryError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    InvalidAmount = 4,
    InvalidMaxMembers = 5,
    InvalidDeadline = 6,
    InvalidTemplateParams = 7,
    TemplateAlreadyRegistered = 8,
    TemplateNotFound = 9,
    InvalidVersion = 10,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TemplateMetadata {
    pub template_id: Symbol,
    pub version: Symbol,
    pub wasm_hash: BytesN<32>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AjoParams {
    pub amount: i128,
    pub max_members: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SavingsParams {
    pub target_amount: i128,
    pub deadline: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EscrowParams {
    pub beneficiary: Address,
    pub amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TemplateParams {
    Ajo(AjoParams),
    Savings(SavingsParams),
    Escrow(EscrowParams),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeployedInstance {
    pub template_id: Symbol,
    pub version: Symbol,
    pub address: Address,
    pub creator: Address,
}

// ---------------------------------------------------------------------------
// Ajo Contract
// ---------------------------------------------------------------------------

#[contract]
pub struct Ajo;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AjoDataKey {
    Amount,
    MaxMembers,
    Creator,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SavingsDataKey {
    Owner,
    TargetAmount,
    Deadline,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EscrowDataKey {
    Depositor,
    Beneficiary,
    Amount,
}

#[contractimpl]
impl Ajo {
    pub fn init_ajo(
        env: Env,
        amount: i128,
        max_members: u32,
        creator: Address,
    ) -> Result<(), FactoryError> {
        if env.storage().instance().has(&AjoDataKey::Creator) {
            return Err(FactoryError::AlreadyInitialized);
        }
        validate_ajo_params(amount, max_members)?;

        env.storage().instance().set(&AjoDataKey::Amount, &amount);
        env.storage()
            .instance()
            .set(&AjoDataKey::MaxMembers, &max_members);
        env.storage().instance().set(&AjoDataKey::Creator, &creator);

        Ok(())
    }

    pub fn ajo_creator(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&AjoDataKey::Creator)
            .expect("Not initialized")
    }

    pub fn ajo_amount(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&AjoDataKey::Amount)
            .expect("Not initialized")
    }

    pub fn ajo_max_members(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&AjoDataKey::MaxMembers)
            .expect("Not initialized")
    }

    pub fn init_savings(
        env: Env,
        target_amount: i128,
        deadline: u64,
        owner: Address,
    ) -> Result<(), FactoryError> {
        if env.storage().instance().has(&SavingsDataKey::Owner) {
            return Err(FactoryError::AlreadyInitialized);
        }
        validate_savings_params(target_amount, deadline)?;

        env.storage().instance().set(&SavingsDataKey::Owner, &owner);
        env.storage()
            .instance()
            .set(&SavingsDataKey::TargetAmount, &target_amount);
        env.storage()
            .instance()
            .set(&SavingsDataKey::Deadline, &deadline);

        Ok(())
    }

    pub fn savings_owner(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&SavingsDataKey::Owner)
            .expect("Not initialized")
    }

    pub fn savings_target_amount(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&SavingsDataKey::TargetAmount)
            .expect("Not initialized")
    }

    pub fn savings_deadline(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&SavingsDataKey::Deadline)
            .expect("Not initialized")
    }

    pub fn init_escrow(
        env: Env,
        depositor: Address,
        beneficiary: Address,
        amount: i128,
    ) -> Result<(), FactoryError> {
        if env.storage().instance().has(&EscrowDataKey::Depositor) {
            return Err(FactoryError::AlreadyInitialized);
        }
        validate_amount(amount)?;

        env.storage()
            .instance()
            .set(&EscrowDataKey::Depositor, &depositor);
        env.storage()
            .instance()
            .set(&EscrowDataKey::Beneficiary, &beneficiary);
        env.storage()
            .instance()
            .set(&EscrowDataKey::Amount, &amount);

        Ok(())
    }

    pub fn escrow_depositor(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&EscrowDataKey::Depositor)
            .expect("Not initialized")
    }

    pub fn escrow_beneficiary(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&EscrowDataKey::Beneficiary)
            .expect("Not initialized")
    }

    pub fn escrow_amount(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&EscrowDataKey::Amount)
            .expect("Not initialized")
    }
}

// ---------------------------------------------------------------------------
// AjoFactory Contract
// ---------------------------------------------------------------------------

#[contract]
pub struct AjoFactory;

#[contracttype]
pub enum FactoryDataKey {
    Admin,
    Template(Symbol, Symbol), // (template_id, version)
    Instances(Address),       // creator -> list of deployed instances
}

#[contractimpl]
impl AjoFactory {
    pub fn initialize(env: Env, admin: Address) -> Result<(), FactoryError> {
        if env.storage().instance().has(&FactoryDataKey::Admin) {
            return Err(FactoryError::AlreadyInitialized);
        }
        env.storage().instance().set(&FactoryDataKey::Admin, &admin);
        Ok(())
    }

    pub fn register_template(
        env: Env,
        admin: Address,
        template_id: Symbol,
        version: Symbol,
        wasm_hash: BytesN<32>,
    ) -> Result<(), FactoryError> {
        admin.require_auth();
        let current_admin: Address = env
            .storage()
            .instance()
            .get(&FactoryDataKey::Admin)
            .ok_or(FactoryError::NotInitialized)?;
        if admin != current_admin {
            return Err(FactoryError::Unauthorized);
        }

        let key = FactoryDataKey::Template(template_id.clone(), version.clone());
        if env.storage().instance().has(&key) {
            return Err(FactoryError::TemplateAlreadyRegistered);
        }

        let metadata = TemplateMetadata {
            template_id,
            version,
            wasm_hash,
        };
        env.storage().instance().set(&key, &metadata);
        Ok(())
    }

    pub fn deploy_template(
        env: Env,
        creator: Address,
        template_id: Symbol,
        version: Symbol,
        params: TemplateParams,
    ) -> Result<Address, FactoryError> {
        creator.require_auth();

        let metadata: TemplateMetadata = env
            .storage()
            .instance()
            .get(&FactoryDataKey::Template(
                template_id.clone(),
                version.clone(),
            ))
            .ok_or(FactoryError::TemplateNotFound)?;

        let salt = env.crypto().sha256(&creator.to_val().into_val(&env));
        let address = env
            .deployer()
            .with_address(creator.clone(), salt)
            .deploy(metadata.wasm_hash);

        match (template_id.clone(), params) {
            (TEMPLATE_AJO, TemplateParams::Ajo(p)) => {
                env.invoke_contract::<()>(
                    &address,
                    &symbol_short!("init_ajo"),
                    (p.amount, p.max_members, creator.clone()).into_val(&env),
                );
            }
            (TEMPLATE_SAVINGS, TemplateParams::Savings(p)) => {
                env.invoke_contract::<()>(
                    &address,
                    &symbol_short!("init_sav"),
                    (p.target_amount, p.deadline, creator.clone()).into_val(&env),
                );
            }
            (TEMPLATE_ESCROW, TemplateParams::Escrow(p)) => {
                env.invoke_contract::<()>(
                    &address,
                    &symbol_short!("init_esc"),
                    (creator.clone(), p.beneficiary, p.amount).into_val(&env),
                );
            }
            _ => return Err(FactoryError::InvalidTemplateParams),
        }

        let mut instances: Vec<DeployedInstance> = env
            .storage()
            .persistent()
            .get(&FactoryDataKey::Instances(creator.clone()))
            .unwrap_or(Vec::new(&env));

        instances.push_back(DeployedInstance {
            template_id,
            version,
            address: address.clone(),
            creator,
        });

        env.storage()
            .persistent()
            .set(&FactoryDataKey::Instances(creator.clone()), &instances);

        Ok(address)
    }

    pub fn get_instances(env: Env, creator: Address) -> Vec<DeployedInstance> {
        env.storage()
            .persistent()
            .get(&FactoryDataKey::Instances(creator))
            .unwrap_or(Vec::new(&env))
    }

    pub fn get_template(
        env: Env,
        template_id: Symbol,
        version: Symbol,
    ) -> Option<TemplateMetadata> {
        env.storage()
            .instance()
            .get(&FactoryDataKey::Template(template_id, version))
    }
}

// ---------------------------------------------------------------------------
// Internal Validation
// ---------------------------------------------------------------------------

fn validate_amount(amount: i128) -> Result<(), FactoryError> {
    if amount <= 0 {
        return Err(FactoryError::InvalidAmount);
    }
    Ok(())
}

fn validate_ajo_params(amount: i128, max_members: u32) -> Result<(), FactoryError> {
    validate_amount(amount)?;
    if max_members < 2 || max_members > 100 {
        return Err(FactoryError::InvalidMaxMembers);
    }
    Ok(())
}

fn validate_savings_params(target_amount: i128, deadline: u64) -> Result<(), FactoryError> {
    validate_amount(target_amount)?;
    if deadline == 0 {
        return Err(FactoryError::InvalidDeadline);
    }
    Ok(())
}
