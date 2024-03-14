use std::collections::HashMap;

use futures::future::join_all;
use serde::{Deserialize, Serialize};

use crate::{
    api::{currencies::get_supported_currencies_from_api, exchange_rate::get_exchange_rates},
    error::print_info,
};

pub mod history;

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

pub async fn get_supported_currencies() -> Result<Vec<Currency>, String> {
    let currencies = get_supported_currencies_from_api().await;

    match currencies {
        Ok(currencies) => Ok(currencies),
        Err(e) => Err(e),
    }
}

pub async fn get_rate(source_currency: &str, target_currency: &str) -> Result<f64, String> {
    let source_currency = Currency::new_from_code(source_currency.to_string());
    let target_currency = Currency::new_from_code(target_currency.to_string());

    let rates = get_exchange_rates(source_currency).await;

    match rates {
        Ok(rates) => {
            let currency_key = target_currency.get_code();
            let rate = rates.get(currency_key);

            match rate {
                Some(rate) => Ok(*rate),
                None => Err(format!(
                    "Exchange rate for {} not found",
                    target_currency.get_code()
                )),
            }
        }
        Err(e) => Err(e),
    }
}

pub async fn get_supported_currencies_with_rates(
) -> Result<Vec<HashMap<String, HashMap<String, f64>>>, String> {
    let currencies = get_supported_currencies_from_api().await;

    print_info("Getting exchange rates for supported currencies...");

    match currencies {
        Ok(currencies) => {
            let mut currencies_with_rates: Vec<HashMap<String, HashMap<String, f64>>> = vec![];

            let get_rates_functions = currencies.iter().map(|currency| {
                let currency_code = currency.get_code().clone();
                async move {
                    let rates = get_exchange_rates(currency.clone()).await;
                    (currency_code, rates)
                }
            });

            let results = join_all(get_rates_functions).await;

            for result in results {
                match result {
                    (currency_code, Ok(rates)) => {
                        let mut currency_with_rates = HashMap::new();
                        currency_with_rates.insert(currency_code, rates);
                        currencies_with_rates.push(currency_with_rates);
                    }
                    (_, Err(e)) => return Err(e),
                }
            }

            Ok(currencies_with_rates)
        }
        Err(e) => Err(e),
    }
}
