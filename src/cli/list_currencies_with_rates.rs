use crate::{currency::get_supported_currencies_with_rates, error::print_error};
use colored::Colorize;

pub async fn display_supported_currencies_with_rates() {
    let supported_currencies_with_rates = get_supported_currencies_with_rates().await;

    match supported_currencies_with_rates {
        Ok(currencies) => {
            for currency in currencies {
                for (code, rates) in currency {
                    println!("{}", code.green());
                    for (index, (target, rate)) in rates.iter().enumerate() {
                        if index == 0 || index % 5 == 0{
                            print!("{}: {}", target.yellow(), rate);
                        } else {
                            print!(" {}: {}", target.yellow(), rate);
                        }
                        if (index + 1) % 5 == 0 {
                            println!("");
                        }
                    }
                    println!("");
                    println!("");
                }
            }
        }
        Err(e) => {
            print_error(e.to_string().as_str());
        }
    }
}
