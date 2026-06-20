use crate::errors::AppError;
use axum::{extract::Request, middleware::Next, response::Response};
use std::env;
use std::sync::LazyLock;

static API_GATEWAY_KEY: LazyLock<String> =
    LazyLock::new(|| env::var("API_GATEWAY_KEY").unwrap_or_default());

static API_GATEWAY_ERROR_MESSAGE: LazyLock<String> = LazyLock::new(|| {
    env::var("API_GATEWAY_ERROR_MESSAGE").unwrap_or_else(|_| "Invalid API Gateway Key".to_string())
});

pub async fn verify_api_gateway_key(req: Request, next: Next) -> Result<Response, AppError> {
    let expected_key = &*API_GATEWAY_KEY;
    let error_message = &*API_GATEWAY_ERROR_MESSAGE;

    if expected_key.is_empty() {
        return Ok(next.run(req).await);
    }

    if let Some(provided_key) = req.headers().get("x-api-gateway-key")
        && let Ok(key_str) = provided_key.to_str()
    {
        use subtle::ConstantTimeEq;
        if key_str.as_bytes().ct_eq(expected_key.as_bytes()).into() {
            return Ok(next.run(req).await);
        }
    }

    Err(AppError::Authorization(error_message.clone()))
}
