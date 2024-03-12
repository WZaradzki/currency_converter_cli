use std::thread;

use crate::{
    cache::{
        file_cache::{create_cache_file, read_and_invalid_cache_file},
        CacheConfigs,
    },
    currency::Currency,
};

pub mod currencies;
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

    fn get_cache_config(&self) -> CacheConfigs {
        match self {
            ApiEndpoints::SupportedCurrencies => CacheConfigs::Currencies,
            ApiEndpoints::ExchangeRate => CacheConfigs::ExchangeRates,
        }
    }

    pub fn get_url(&self, currency: Option<Currency>) -> String {
        let url = self.prepare_url();
        match currency {
            Some(currency) => format!("{}/{}", url, currency.get_code()),
            None => url,
        }
    }

    pub async fn request<
        T: for<'de> serde::Deserialize<'de> + serde::Serialize + Clone + Send + 'static,
    >(
        &self,
        currency: Option<Currency>,
    ) -> Result<T, reqwest::Error> {
        let cache_config = self.get_cache_config();
        let cached_response = read_and_invalid_cache_file(cache_config.clone(), currency.clone());

        match cached_response {
            Ok(cached_response) => Ok(cached_response),
            Err(_) => {
                let response = reqwest::get(&self.get_url(currency.clone())).await;

                let response = match response {
                    Ok(response) => response.json::<T>().await,
                    Err(e) => {
                        return Err(e);
                    }
                };

                let currencies = match response {
                    Ok(response) => {
                        let cloned_response = response.clone();
                        thread::spawn(move || {
                            let _ = create_cache_file(&cloned_response, cache_config, currency);
                        });
                        response
                    }
                    Err(e) => {
                        return Err(e);
                    }
                };

                Ok(currencies)
            }
        }
    }
}
