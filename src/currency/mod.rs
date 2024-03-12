use crate::api::{exchange_rate::get_exchange_rates, Currency};


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