use goose::prelude::*;
use goose_eggs::{validate_and_load_static_assets, Validate};

async fn loadtest_index(user: &mut GooseUser) -> TransactionResult {
    let goose = user.get("/profile").await?;
    let validate = &Validate::builder().status(200).build();
    validate_and_load_static_assets(user, goose, validate).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(
            scenario!("LoadtestTransactions").register_transaction(transaction!(loadtest_index)),
        )
        .execute()
        .await?;
    Ok(())
}
