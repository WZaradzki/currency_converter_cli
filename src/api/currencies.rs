use serde::{Deserialize, Serialize};

use crate::currency::Currency;

use super::ApiEndpoints;

pub async fn get_supported_currencies_from_api() -> Result<Vec<Currency>, String> {
    let endpoint = ApiEndpoints::SupportedCurrencies;
    let response: Result<ApiResponse, reqwest::Error> = endpoint.request(None).await;

    match response {
        Ok(response) => Ok(response.supported_codes),
        Err(e) => Err(e.to_string()),
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ApiResponse {
    result: String,
    documentation: String,
    terms_of_use: String,
    supported_codes: Vec<Currency>,
}
#[derive(Deserialize, Serialize, Debug)]
struct ApiError {
    result: String,
    error_type: ErrorType,
}
#[derive(Deserialize, Serialize, Debug)]
enum ErrorType {
    InvalidKey,
    InactiveAccount,
    QuotaReached,
}
