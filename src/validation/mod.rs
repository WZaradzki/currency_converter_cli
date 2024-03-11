
pub enum ValidationType {
    Amount,
    Currency,
}

pub async fn validate(currency: &String, validation_type: ValidationType) -> Result<(), String> {
    let valid_currencies = [
        "USD", "EUR", "JPY", "GBP", "AUD", "CAD", "CHF", "CNY", "SEK", "NZD",
    ];

    return match validation_type {
        ValidationType::Amount => {
            if currency.parse::<f64>().is_ok() {
                Ok(())
            } else {
                Err(format!("{} is not a valid number", currency))
            }
        }
        ValidationType::Currency => {
            if valid_currencies.contains(&currency.as_str()) {
                Ok(())
            } else {
                Err(format!("{} is not a valid currency", currency))
            }
        }
        _ => Err("Invalid type".to_string()),
    };
}
