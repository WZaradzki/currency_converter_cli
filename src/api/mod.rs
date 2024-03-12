
use std::thread;

use serde::{Deserialize, Serialize};

use crate::{
    cache::{
        file_cache::{create_cache_file, read_and_invalid_cache_file},
        CacheConfigs,
    },
    error::print_warning,
};

pub mod exchange_rate;

enum ApiEndpoints {
    SupportedCurrencies,
    ExchangeRate,
}

impl ApiEndpoints {
    fn prepare_url(&self) -> String {
        dotenv::dotenv().ok();
        let api_key: String = std::env::var("API_KEY").unwrap();

        match self {
            ApiEndpoints::SupportedCurrencies => {
                format!("https://v6.exchangerate-api.com/v6/{}/codes", api_key)
            }
            ApiEndpoints::ExchangeRate => {
                format!("https://v6.exchangerate-api.com/v6/{}/latest/", api_key)
            }
        }
    }

    pub fn get_url(&self) -> String {
        self.prepare_url()
    }
}

pub async fn get_supported_currencies() -> Result<Vec<Currency>, reqwest::Error> {
    let endpoint = ApiEndpoints::SupportedCurrencies;
    let url = endpoint.get_url();

    let cached_response = read_and_invalid_cache_file(CacheConfigs::Currencies, None);
    match cached_response {
        Ok(cached_response) => Ok(cached_response),
        Err(e) => {
            print_warning(&e.to_string());
            // panic!("cached_response:");

            let response = reqwest::get(&url).await;

            let response = match response {
                Ok(response) => response.json::<ApiResponse>().await,
                Err(e) => {
                    return Err(e);
                }
            };

            let currencies = match response {
                Ok(response) => {
                    let supported_codes_clone = response.supported_codes.clone();
                    thread::spawn(move || {
                        let _ = create_cache_file(
                            &supported_codes_clone,
                            CacheConfigs::Currencies,
                            None,
                        );
                    });
                    response.supported_codes
                }
                Err(e) => {
                    return Err(e);
                }
            };

            Ok(currencies)
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct ApiResponse {
    result: String,
    documentation: String,
    terms_of_use: String,
    supported_codes: Vec<Currency>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Currency {
    code: String,
    name: Option<String>,
}
impl Currency {
    pub fn new_from_code(code: String) -> Currency {
        Currency {
            code: code.to_uppercase(),
            name: None,
        }
    }

    pub fn get_code(&self) -> &String {
        &self.code
    }

    pub fn get_name(&self) -> &String {
        let name = self.name.as_ref();
        if name.is_none() {
            &self.code
        } else {
            name.unwrap()
        }
    }
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
