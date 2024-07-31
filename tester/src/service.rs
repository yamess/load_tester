use goose::prelude::*;
use crate::schemas::LoadTestPayload;

pub async fn loadtest_healthcheck(user: &mut GooseUser) -> TransactionResult {
    let _ = user.get("/api/v1").await?;
    Ok(())
}

pub async fn loadtest_predict(user: &mut GooseUser) -> TransactionResult {
    let payload = &serde_json::json!({
        "x1": 1.2,
        "x2": 0.5
    });
    let _ = user.post_json("/api/v1/predict", &payload).await?;
    Ok(())
}

pub async fn run_load_test(payload: LoadTestPayload, file_path: String) -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(
            scenario!("Loadtest healthcheck")
                .register_transaction(transaction!(loadtest_healthcheck))
                .register_transaction(transaction!(loadtest_predict))
        )
        .set_default(GooseDefault::Host, payload.host.as_str())?
        .set_default(GooseDefault::Users, payload.users)?
        .set_default(GooseDefault::RunTime, payload.runtime)?
        .set_default(GooseDefault::ReportFile, file_path.as_str())?
        .set_default(GooseDefault::NoResetMetrics, true)?

        .execute()
        .await?;

    Ok(())
}