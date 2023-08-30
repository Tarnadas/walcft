use super::{event, log_tx_result};
use near_sdk::json_types::U128;
use serde::Serialize;
use serde_json::json;
use workspaces::{
    result::{ExecutionResult, Value},
    types::Balance,
    Account, AccountId, Contract,
};

// pub async fn migrate(
//     contract: &Contract,
//     sender: &Account,
// ) -> anyhow::Result<ExecutionResult<Value>> {
//     let (res, _): (ExecutionResult<Value>, Vec<event::ContractEvent>) = log_tx_result(
//         Some("migrate"),
//         sender
//             .call(contract.id(), "migrate")
//             .max_gas()
//             .transact()
//             .await?,
//     )?;
//     Ok(res)
// }

pub async fn storage_deposit(
    contract: &Contract,
    sender: &Account,
    account_id: Option<&AccountId>,
    registration_only: Option<bool>,
    deposit: Option<Balance>,
) -> anyhow::Result<ExecutionResult<Value>> {
    let (res, _) = log_tx_result(
        Some("storage_deposit"),
        sender
            .call(contract.id(), "storage_deposit")
            .args_json((account_id, registration_only))
            .deposit(deposit.unwrap_or(10_000_000_000_000_000_000_000))
            .max_gas()
            .transact()
            .await?,
    )?;
    Ok(res)
}

pub async fn ft_transfer(
    sender: &Account,
    token_id: &AccountId,
    receiver_id: &AccountId,
    amount: u128,
) -> anyhow::Result<ExecutionResult<Value>> {
    let (res, _) = log_tx_result(
        Some("ft_transfer"),
        sender
            .call(token_id, "ft_transfer")
            .args_json((receiver_id, U128(amount), Option::<String>::None))
            .max_gas()
            .deposit(1)
            .transact()
            .await?,
    )?;
    Ok(res)
}
async fn ft_transfer_call<T: Serialize>(
    sender: &Account,
    token_id: &AccountId,
    receiver_id: &AccountId,
    amount: U128,
    msg: T,
) -> anyhow::Result<ExecutionResult<Value>> {
    let (res, _) = log_tx_result(
        Some("ft_transfer_call"),
        sender
            .call(token_id, "ft_transfer_call")
            .args_json((
                receiver_id,
                amount,
                Option::<String>::None,
                json!(msg).to_string(),
            ))
            .max_gas()
            .deposit(1)
            .transact()
            .await?,
    )?;
    Ok(res)
}
