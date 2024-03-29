use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{currency::Currency, error::print_info};

use super::ApiEndpoints;

pub async fn get_exchange_rates(source: Currency) -> Result<HashMap<String, f64>, String> {
    print_info(&format!(
        "Getting exchange rates for currency {}",
        source.get_code(),
    ));

    let endpoint = ApiEndpoints::ExchangeRate;
    let response: Result<CurrencyRatesApiResponse, String> = endpoint.request(Some(source)).await;

    match response {
        Ok(response) => Ok(response.conversion_rates),
        Err(e) => Err(e.to_string()),
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct CurrencyRatesApiResponse {
    conversion_rates: HashMap<String, f64>,
}
