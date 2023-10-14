use super::{log_tx_result, Action, DaoConfig, DaoPolicy, ProposalInput};
use near_sdk::json_types::{Base58CryptoHash, U128};
use workspaces::{
    result::{ExecutionResult, Value},
    types::Balance,
    Account, AccountId, Contract,
};

pub async fn migrate_old(
    contract: &Contract,
    sender: &Account,
    owner: &AccountId,
) -> anyhow::Result<ExecutionResult<Value>> {
    log_tx_result(
        Some("migrate_old"),
        sender
            .call(contract.id(), "migrate")
            .args_json((owner,))
            .max_gas()
            .transact()
            .await?,
    )
}

// pub async fn migrate(
//     contract: &Contract,
//     sender: &Account,
// ) -> anyhow::Result<ExecutionResult<Value>> {
//     log_tx_result(
//         Some("migrate"),
//         sender
//             .call(contract.id(), "migrate")
//             .max_gas()
//             .transact()
//             .await?,
//     )
// }

pub async fn storage_deposit(
    contract: &Contract,
    sender: &Account,
    account_id: Option<&AccountId>,
    registration_only: Option<bool>,
    deposit: Option<Balance>,
) -> anyhow::Result<ExecutionResult<Value>> {
    log_tx_result(
        Some("storage_deposit"),
        sender
            .call(contract.id(), "storage_deposit")
            .args_json((account_id, registration_only))
            .deposit(deposit.unwrap_or(10_000_000_000_000_000_000_000))
            .max_gas()
            .transact()
            .await?,
    )
}

pub async fn ft_transfer(
    sender: &Account,
    token_id: &AccountId,
    receiver_id: &AccountId,
    amount: u128,
) -> anyhow::Result<ExecutionResult<Value>> {
    log_tx_result(
        Some("ft_transfer"),
        sender
            .call(token_id, "ft_transfer")
            .args_json((receiver_id, U128(amount), Option::<String>::None))
            .max_gas()
            .deposit(1)
            .transact()
            .await?,
    )
}

pub async fn new_dao(
    contract: &Contract,
    config: DaoConfig,
    policy: DaoPolicy,
) -> anyhow::Result<ExecutionResult<Value>> {
    log_tx_result(
        Some("DAO: new"),
        contract
            .call("new")
            .args_json((config, policy))
            .max_gas()
            .transact()
            .await?,
    )
}

pub async fn store_blob(
    sender: &Account,
    dao: &AccountId,
    blob: Vec<u8>,
    storage_cost: u128,
) -> anyhow::Result<Base58CryptoHash> {
    Ok(log_tx_result(
        Some("DAO: store_blob"),
        sender
            .call(dao, "store_blob")
            .args(blob)
            .max_gas()
            .deposit(storage_cost)
            .transact()
            .await?,
    )?
    .json()?)
}

pub async fn add_proposal(
    sender: &Account,
    dao: &AccountId,
    proposal: ProposalInput,
    deposit: Option<u128>,
) -> anyhow::Result<u64> {
    Ok(log_tx_result(
        Some("DAO: add_proposal"),
        sender
            .call(dao, "add_proposal")
            .args_json((proposal,))
            .max_gas()
            .deposit(deposit.unwrap_or(1_000_000_000_000_000_000_000_000))
            .transact()
            .await?,
    )?
    .json()?)
}

pub async fn act_proposal(
    sender: &Account,
    dao: &AccountId,
    proposal_id: u64,
    action: Action,
) -> anyhow::Result<ExecutionResult<Value>> {
    log_tx_result(
        Some("DAO: act_proposal"),
        sender
            .call(dao, "act_proposal")
            .args_json((proposal_id, action, None::<String>))
            .max_gas()
            .transact()
            .await?,
    )
}
