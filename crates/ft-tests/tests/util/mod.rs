pub mod call;
pub mod event;
pub mod view;

use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FT_METADATA_SPEC};
use near_sdk::json_types::U128;
use owo_colors::OwoColorize;
use tokio::fs;
use workspaces::{
    network::Sandbox,
    result::{ExecutionFinalResult, ExecutionResult, Value, ViewResultDetails},
    types::{KeyType, SecretKey},
    Account, Contract, Worker,
};

#[macro_export]
macro_rules! print_log {
    ( $x:expr, $($y:expr),+ ) => {
        let thread_name = std::thread::current().name().unwrap().to_string();
        if thread_name == "main" {
            println!($x, $($y),+);
        } else {
            let mut s = format!($x, $($y),+);
            s = s.split('\n').map(|s| {
                let mut pre = "    ".to_string();
                pre.push_str(s);
                pre.push('\n');
                pre
            }).collect::<String>();
            println!(
                "{}\n{}",
                thread_name.bold(),
                &s[..s.len() - 1],
            );
        }
    };
}

pub async fn initialize_contracts(
    total_supply: u128,
    path: Option<&'static str>,
) -> anyhow::Result<(Worker<Sandbox>, Account, Contract)> {
    let worker = workspaces::sandbox().await?;

    let owner = worker.dev_create_account().await?;

    let key = SecretKey::from_random(KeyType::ED25519);
    let contract = worker
        .create_tla_and_deploy(
            "ft.test.near".parse()?,
            key,
            &fs::read(path.unwrap_or("../../out/fungible_token.wasm")).await?,
        )
        .await?
        .into_result()?;

    contract
        .call("new")
        .args_json((
            owner.id(),
            U128(total_supply),
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "WALC".to_string(),
                symbol: "WALC".to_string(),
                icon: None,
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        ))
        .max_gas()
        .transact()
        .await?
        .into_result()?;

    Ok((worker, owner, contract))
}

pub fn log_tx_result(
    ident: Option<&str>,
    res: ExecutionFinalResult,
) -> anyhow::Result<ExecutionResult<Value>> {
    for failure in res.receipt_failures() {
        print_log!("{:#?}", failure.bright_red());
    }
    for outcome in res.receipt_outcomes() {
        if !outcome.logs.is_empty() {
            for log in outcome.logs.iter() {
                if log.starts_with("EVENT_JSON:") {
                    let event: event::ContractEvent =
                        serde_json::from_str(&log.replace("EVENT_JSON:", ""))?;
                    print_log!(
                        "{}: {}\n{}",
                        "account".bright_cyan(),
                        outcome.executor_id,
                        event
                    );
                } else {
                    print_log!("{}", log.bright_yellow());
                }
            }
        }
    }
    if let Some(ident) = ident {
        print_log!(
            "{} gas burnt: {:.3} {}",
            ident.italic(),
            (res.total_gas_burnt as f64 / 1_000_000_000_000.)
                .bright_magenta()
                .bold(),
            "TGas".bright_magenta().bold()
        );
    }
    Ok(res.into_result()?)
}

pub fn log_view_result(res: ViewResultDetails) -> anyhow::Result<ViewResultDetails> {
    if !res.logs.is_empty() {
        for log in res.logs.iter() {
            print_log!("{}", log.bright_yellow());
        }
    }
    Ok(res)
}
