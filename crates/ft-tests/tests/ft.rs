mod util;

use util::*;

const TOTAL_SUPPLY: u128 = 100_000_000_000_000_000_000_000_000;

#[tokio::test]
async fn test_migrate() -> anyhow::Result<()> {
    let (worker, owner, contract) =
        initialize_contracts(TOTAL_SUPPLY, Some("../../out/fungible_token.wasm")).await?;

    let user_0 = worker.dev_create_account().await?;
    let user_1 = worker.dev_create_account().await?;
    let user_2 = worker.dev_create_account().await?;

    tokio::try_join!(
        call::storage_deposit(&contract, &user_0, None, Some(true), None),
        call::storage_deposit(&contract, &user_1, None, Some(true), None),
        call::storage_deposit(&contract, &user_2, None, Some(true), None)
    )?;

    call::ft_transfer(&owner, contract.id(), user_0.id(), 100).await?;
    call::ft_transfer(&owner, contract.id(), user_1.id(), 200).await?;
    call::ft_transfer(&owner, contract.id(), user_2.id(), 300).await?;

    let balance = view::ft_balance_of(&contract, user_0.id()).await?;
    assert_eq!(balance.0, 100);
    let balance = view::ft_balance_of(&contract, user_1.id()).await?;
    assert_eq!(balance.0, 200);
    let balance = view::ft_balance_of(&contract, user_2.id()).await?;
    assert_eq!(balance.0, 300);
    let balance = view::ft_balance_of(&contract, owner.id()).await?;
    assert_eq!(balance.0, TOTAL_SUPPLY - 600);

    call::migrate(&contract, contract.as_account()).await?;

    let balance = view::ft_balance_of(&contract, user_0.id()).await?;
    assert_eq!(balance.0, 100);
    let balance = view::ft_balance_of(&contract, user_1.id()).await?;
    assert_eq!(balance.0, 200);
    let balance = view::ft_balance_of(&contract, user_2.id()).await?;
    assert_eq!(balance.0, 300);
    let balance = view::ft_balance_of(&contract, owner.id()).await?;
    assert_eq!(balance.0, TOTAL_SUPPLY - 600);

    Ok(())
}
