use sentrystr_tracing::SentryStrTracingBuilder;
use tracing::{error, info, warn, Instrument};

#[derive(Debug)]
struct User {
    id: u64,
    email: String,
    role: String,
}

async fn authenticate_user(user_id: u64) -> Result<User, String> {
    let span = tracing::info_span!("authenticate_user", user_id = user_id);

    async move {
        info!("Starting user authentication");

        if user_id == 0 {
            error!(
                user_id = user_id,
                error = "invalid_user_id",
                "Authentication failed: invalid user ID"
            );
            return Err("Invalid user ID".to_string());
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let user = User {
            id: user_id,
            email: format!("user{}@example.com", user_id),
            role: "user".to_string(),
        };

        info!(
            user_id = user.id,
            email = %user.email,
            role = %user.role,
            "User authenticated successfully"
        );

        Ok(user)
    }
    .instrument(span)
    .await
}

async fn process_payment(amount: f64, currency: &str) -> Result<String, String> {
    let span = tracing::info_span!("process_payment", amount = amount, currency = currency);

    async move {
        info!("Processing payment");

        if amount <= 0.0 {
            error!(
                amount = amount,
                currency = currency,
                error = "invalid_amount",
                "Payment failed: invalid amount"
            );
            return Err("Invalid amount".to_string());
        }

        if amount > 10000.0 {
            warn!(
                amount = amount,
                currency = currency,
                threshold = 10000.0,
                "Large payment detected, requires manual review"
            );
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        let transaction_id = uuid::Uuid::new_v4().to_string();

        info!(
            transaction_id = %transaction_id,
            amount = amount,
            currency = currency,
            status = "completed",
            "Payment processed successfully"
        );

        Ok(transaction_id)
    }
    .instrument(span)
    .await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let relays = vec![
        "wss://relay.damus.io".to_string(),
        "wss://nos.lol".to_string(),
    ];

    SentryStrTracingBuilder::new()
        .with_generated_keys_and_relays(relays)
        .with_min_level(tracing::Level::INFO)
        .with_fields(true)
        .with_metadata(true)
        .init()
        .await?;

    info!(
        service = "payment_processor",
        version = "2.1.0",
        environment = "production",
        "Service started"
    );

    match authenticate_user(12345).await {
        Ok(user) => {
            info!(
                user_id = user.id,
                operation = "authentication",
                duration_ms = 150,
                "Authentication completed"
            );

            match process_payment(150.75, "USD").await {
                Ok(tx_id) => {
                    info!(
                        transaction_id = %tx_id,
                        user_id = user.id,
                        "Transaction successful"
                    );
                }
                Err(e) => {
                    error!(
                        user_id = user.id,
                        error = %e,
                        "Payment processing failed"
                    );
                }
            }
        }
        Err(e) => {
            error!(
                error = %e,
                "Authentication failed"
            );
        }
    }

    match authenticate_user(0).await {
        Ok(_) => unreachable!(),
        Err(e) => {
            warn!(
                error = %e,
                "Expected authentication failure occurred"
            );
        }
    }

    let large_payment_result = process_payment(15000.0, "EUR").await;
    match large_payment_result {
        Ok(tx_id) => {
            info!(
                transaction_id = %tx_id,
                flags = ?["large_amount", "manual_review"],
                "Large payment completed"
            );
        }
        Err(e) => {
            error!(
                error = %e,
                "Large payment failed"
            );
        }
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    info!(
        shutdown_reason = "graceful",
        uptime_seconds = 30,
        "Service shutting down"
    );

    Ok(())
}
