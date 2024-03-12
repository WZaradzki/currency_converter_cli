use std::{collections::HashMap, thread};

use serde::Deserialize;

use crate::{
    cache::{
        file_cache::{create_cache_file, read_and_invalid_cache_file},
        CacheConfigs,
    },
    error::print_warning,
};

use super::{ApiEndpoints, Currency};

pub async fn get_exchange_rates(source: Currency) -> Result<HashMap<String, f64>, String> {
    let cached_response =
        read_and_invalid_cache_file(CacheConfigs::ExchangeRates, Some(source.clone()));
    match cached_response {
        Ok(cached_response) => Ok(cached_response),
        Err(e) => {
            print_warning(&e.to_string());

            let endpoint = ApiEndpoints::ExchangeRate;
            let url = endpoint.get_url() + source.get_code().as_str();

            let response = reqwest::get(&url).await;

            let response = match response {
                Ok(response) => response.json::<CurrencyRatesApiResponse>().await,
                Err(e) => {
                    return Err(e.to_string());
                }
            };

            let rates = match response {
                Ok(response) => {
                    let conversion_rates = response.conversion_rates.clone();
                    thread::spawn(move || {
                        let _ = create_cache_file(
                            &conversion_rates,
                            CacheConfigs::ExchangeRates,
                            Some(source),
                        );
                    });
                    response.conversion_rates
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            };

            Ok(rates)
        }
    }
}

#[derive(Deserialize)]
struct CurrencyRatesApiResponse {
    conversion_rates: HashMap<String, f64>,
}
