use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

fn setup() -> (Env, AjoFactoryClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, AjoFactory);
    let client = AjoFactoryClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.initialize(&admin);
    (env, client, admin)
}

#[test]
fn test_register_and_deploy_ajo() {
    let (env, client, admin) = setup();
    let creator = Address::generate(&env);
    let wasm_hash = env.deployer().upload_contract_wasm(Ajo::WASM);

    client.register_template(&admin, &TEMPLATE_AJO, &DEFAULT_VERSION, &wasm_hash);

    let params = TemplateParams::Ajo(AjoParams {
        amount: 1000,
        max_members: 10,
    });

    let address = client.deploy_template(&creator, &TEMPLATE_AJO, &DEFAULT_VERSION, &params);

    let ajo_client = AjoClient::new(&env, &address);
    assert_eq!(ajo_client.ajo_creator(), creator);
    assert_eq!(ajo_client.ajo_amount(), 1000);
    assert_eq!(ajo_client.ajo_max_members(), 10);
}

#[test]
fn test_register_already_exists() {
    let (env, client, admin) = setup();
    let wasm_hash = env.deployer().upload_contract_wasm(Ajo::WASM);

    client.register_template(&admin, &TEMPLATE_AJO, &DEFAULT_VERSION, &wasm_hash);
    let result = client.try_register_template(&admin, &TEMPLATE_AJO, &DEFAULT_VERSION, &wasm_hash);

    assert_eq!(result, Err(Ok(FactoryError::TemplateAlreadyRegistered)));
}

#[test]
fn test_deploy_not_found() {
    let (env, client, _) = setup();
    let creator = Address::generate(&env);
    let params = TemplateParams::Ajo(AjoParams {
        amount: 1000,
        max_members: 10,
    });

    let result = client.try_deploy_template(&creator, &TEMPLATE_AJO, &DEFAULT_VERSION, &params);
    assert_eq!(result, Err(Ok(FactoryError::TemplateNotFound)));
}

#[test]
fn test_unauthorized_registration() {
    let (env, client, _) = setup();
    let attacker = Address::generate(&env);
    let wasm_hash = env.deployer().upload_contract_wasm(Ajo::WASM);

    let result =
        client.try_register_template(&attacker, &TEMPLATE_AJO, &DEFAULT_VERSION, &wasm_hash);
    assert_eq!(result, Err(Ok(FactoryError::Unauthorized)));
}

#[test]
fn test_get_instances() {
    let (env, client, admin) = setup();
    let creator = Address::generate(&env);
    let wasm_hash = env.deployer().upload_contract_wasm(Ajo::WASM);

    client.register_template(&admin, &TEMPLATE_AJO, &DEFAULT_VERSION, &wasm_hash);

    let params = TemplateParams::Ajo(AjoParams {
        amount: 1000,
        max_members: 10,
    });

    client.deploy_template(&creator, &TEMPLATE_AJO, &DEFAULT_VERSION, &params);
    client.deploy_template(&creator, &TEMPLATE_AJO, &DEFAULT_VERSION, &params);

    let instances = client.get_instances(&creator);
    assert_eq!(instances.len(), 2);
}
