mod util;

use near_sdk::env;
use tokio::fs;
use util::*;
use workspaces::types::{KeyType, SecretKey};

// 5 billion
const TOTAL_SUPPLY: u128 = 5_000_000_000_000_000_000_000_000_000_000_000;

#[tokio::test]
async fn test_upgrade_contract_via_dao() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let council = worker.dev_create_account().await?;

    let dao_contract = worker
        .create_tla_and_deploy(
            "dao.test.near".parse()?,
            SecretKey::from_random(KeyType::ED25519),
            &fs::read("./out/sputnik_dao.wasm").await?,
        )
        .await?
        .into_result()?;
    call::new_dao(
        &dao_contract,
        DaoConfig {
            name: "walc".to_string(),
            purpose: "WALC DAO".to_string(),
            metadata: "".to_string(),
        },
        DaoPolicy(vec![council.id().clone()]),
    )
    .await?;

    let contract = initialize_contracts(
        &worker,
        dao_contract.as_account(),
        TOTAL_SUPPLY,
        Some("./out/fungible_token_broken_total_supply.wasm"),
    )
    .await?;

    let user_0 = worker.dev_create_account().await?;
    let user_1 = worker.dev_create_account().await?;
    let user_2 = worker.dev_create_account().await?;

    tokio::try_join!(
        call::storage_deposit(&contract, &user_0, None, Some(true), None),
        call::storage_deposit(&contract, &user_1, None, Some(true), None),
        call::storage_deposit(&contract, &user_2, None, Some(true), None)
    )?;

    call::ft_transfer(dao_contract.as_account(), contract.id(), user_0.id(), 100).await?;
    call::ft_transfer(dao_contract.as_account(), contract.id(), user_1.id(), 200).await?;
    call::ft_transfer(dao_contract.as_account(), contract.id(), user_2.id(), 300).await?;

    let balance = view::ft_balance_of(&contract, user_0.id()).await?;
    assert_eq!(balance.0, 100);
    let balance = view::ft_balance_of(&contract, user_1.id()).await?;
    assert_eq!(balance.0, 200);
    let balance = view::ft_balance_of(&contract, user_2.id()).await?;
    assert_eq!(balance.0, 300);
    let balance = view::ft_balance_of(&contract, dao_contract.id()).await?;
    assert_eq!(balance.0, TOTAL_SUPPLY - 600);

    let total_supply = view::ft_total_supply(&contract).await?;
    assert_eq!(total_supply.0, TOTAL_SUPPLY);

    contract
        .as_account()
        .deploy(include_bytes!("../out/fungible_token_old.wasm"))
        .await?
        .into_result()?;
    call::migrate_old(&contract, contract.as_account(), dao_contract.id()).await?;

    let total_supply = view::ft_total_supply(&contract).await?;
    // total supply is broken and needs to be fixed with DAO upgrade
    assert_eq!(total_supply.0, 0);

    let blob = fs::read("./out/fungible_token.wasm").await?;
    let storage_cost = ((blob.len() + 32) as u128) * env::storage_byte_cost();
    let hash = call::store_blob(&council, dao_contract.id(), blob, storage_cost).await?;

    let proposal_id = call::add_proposal(
        &council,
        dao_contract.id(),
        ProposalInput {
            description: "upgrade contract".to_string(),
            kind: ProposalKind::UpgradeRemote {
                receiver_id: contract.id().clone(),
                method_name: "upgrade".to_string(),
                hash,
            },
        },
        None,
    )
    .await?;
    call::act_proposal(
        &council,
        dao_contract.id(),
        proposal_id,
        Action::VoteApprove,
    )
    .await?;

    let proposal_id = call::add_proposal(
        &council,
        dao_contract.id(),
        ProposalInput {
            description: "migrate contract".to_string(),
            kind: ProposalKind::FunctionCall {
                receiver_id: contract.id().clone(),
                actions: vec![ActionCall {
                    method_name: "migrate".to_string(),
                    args: vec![].into(),
                    deposit: 0.into(),
                    gas: 100_000_000_000_000.into(),
                }],
            },
        },
        None,
    )
    .await?;
    call::act_proposal(
        &council,
        dao_contract.id(),
        proposal_id,
        Action::VoteApprove,
    )
    .await?;

    let balance = view::ft_balance_of(&contract, user_0.id()).await?;
    assert_eq!(balance.0, 100);
    let balance = view::ft_balance_of(&contract, user_1.id()).await?;
    assert_eq!(balance.0, 200);
    let balance = view::ft_balance_of(&contract, user_2.id()).await?;
    assert_eq!(balance.0, 300);
    let balance = view::ft_balance_of(&contract, dao_contract.id()).await?;
    assert_eq!(balance.0, TOTAL_SUPPLY - 600);

    let total_supply = view::ft_total_supply(&contract).await?;
    assert_eq!(total_supply.0, TOTAL_SUPPLY);

    Ok(())
}
