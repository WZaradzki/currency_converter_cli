use crate::{currency::get_supported_currencies_with_rates, error::print_error};
use colored::Colorize;

pub async fn display_supported_currencies_with_rates() {
    let supported_currencies_with_rates = get_supported_currencies_with_rates().await;

    match supported_currencies_with_rates {
        Ok(currencies) => {
            for currency in currencies {
                for (code, rates) in currency {
                    println!("{}", code.green());
                    for (target, rate) in rates {
                        println!("  {} - {}", target, rate);
                    }
                }
            }
        }
        Err(e) => {
            print_error(e.to_string().as_str());
        }
    }
}
