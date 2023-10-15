//1 This file is not used for tests.
//! You can uncomment this file to send proposals to the WALC DAO to do a contract upgrade
#![allow(unused)]

mod util;

use near_sdk::env;
use tokio::fs;
use util::*;
use workspaces::{Account, AccountId};

#[tokio::test]
async fn run_dao_upgrade() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    let worker = workspaces::mainnet().await?;
    let council = Account::from_file(std::env::var("SIGNER_PATH")?, &worker)?;
    let contract_id: AccountId = "walc.near".parse()?;
    let dao_contract_id: AccountId = "walc.sputnik-dao.near".parse()?;

    let blob = fs::read("./out/fungible_token.wasm").await?;
    let storage_cost = ((blob.len() + 32) as u128) * env::storage_byte_cost();
    let hash = call::store_blob(&council, &dao_contract_id, blob, storage_cost).await?;

    let proposal_id = call::add_proposal(
        &council,
        &dao_contract_id,
        ProposalInput {
            description: "Upgrade contract".to_string(),
            kind: ProposalKind::UpgradeRemote {
                receiver_id: contract_id.clone(),
                method_name: "upgrade".to_string(),
                hash,
            },
        },
        Some(100_000_000_000_000_000_000_000),
    )
    .await?;
    call::act_proposal(&council, &dao_contract_id, proposal_id, Action::VoteApprove).await?;

    let proposal_id = call::add_proposal(
        &council,
        &dao_contract_id,
        ProposalInput {
            description: "Migrate contract".to_string(),
            kind: ProposalKind::FunctionCall {
                receiver_id: contract_id,
                actions: vec![ActionCall {
                    method_name: "migrate".to_string(),
                    args: vec![].into(),
                    deposit: 0.into(),
                    gas: 150_000_000_000_000.into(),
                }],
            },
        },
        Some(100_000_000_000_000_000_000_000),
    )
    .await?;
    call::act_proposal(&council, &dao_contract_id, proposal_id, Action::VoteApprove).await?;

    Ok(())
}
