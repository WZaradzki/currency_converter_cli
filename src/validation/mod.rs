use crate::currency::Currency;


pub enum ValidationType {
    Amount,
    Currency,
}

pub async fn validate(
    currency: &String,
    validation_type: ValidationType,
    supported_currencies: Option<&Vec<Currency>>,
) -> Result<(), String> {
    return match validation_type {
        ValidationType::Amount => {
            if currency.parse::<f64>().is_ok() {
                Ok(())
            } else {
                Err(format!("{} is not a valid number", currency))
            }
        }
        ValidationType::Currency => {
            let supported_currencies = supported_currencies.unwrap();
            let currency = currency.to_uppercase();

            if supported_currencies
                .iter()
                .any(|c| c.get_code() == &currency)
            {
                Ok(())
            } else {
                Err(format!("{} is not a valid currency", currency))
            }
        }
    };
}
