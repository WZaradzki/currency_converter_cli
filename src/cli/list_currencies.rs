use crate::{currency::get_supported_currencies, error::print_error};
use colored::Colorize;

pub async fn display_supported_currencies() {
    let supported_currencies = get_supported_currencies().await;

    match supported_currencies {
        Ok(currencies) => {
            for currency in currencies {
                println!("{} - {}", currency.get_code().green(), currency.get_name());
            }
        }
        Err(e) => {
            print_error(e.to_string().as_str());
        }
    }
}
