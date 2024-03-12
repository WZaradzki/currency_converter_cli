use colored::Colorize;
use futures::future::join_all;

use crate::{
    cache::file_cache::rest_cache,
    conversion::convert,
    currency::{get_rate, get_supported_currencies, get_supported_currencies_with_rates},
    error::print_error,
    validation::{validate, ValidationType},
};
pub mod interactive_mode;

#[derive(Debug, PartialEq)]
pub enum Action {
    InteractiveMode,
    DirectConversion {
        source: String,
        target: String,
        amount: String,
    },
    UpdateCache,
    Help,
    Error {
        message: String,
    },
    ListCurrencies,
    ListCurrenciesWithRates,
}

impl Action {
    fn info_print(&self) {
        match self {
            Action::InteractiveMode => println!("Running in interactive mode"),
            Action::UpdateCache => println!("Updating cache"),
            Action::Help => println!("Printing help"),
            Action::Error { message } => println!("{} {}", "Error:".red().bold(), &message.red()),
            Action::ListCurrencies => println!("Listing supported currencies"),
            Action::ListCurrenciesWithRates => println!("Listing supported currencies with rates"),
            _ => (),
        }
    }

    async fn validate(&self) -> Result<(), String> {
        return match self {
            Action::DirectConversion {
                source,
                target,
                amount,
            } => {
                let supported_currencies = get_supported_currencies().await;

                let supported_currencies = match supported_currencies {
                    Ok(currencies) => currencies,
                    Err(_) => {
                        return Err("Failed to get supported currencies".to_string());
                    }
                };

                let validations = vec![
                    validate(&amount, ValidationType::Amount, None),
                    validate(
                        &source,
                        ValidationType::Currency,
                        Some(&supported_currencies),
                    ),
                    validate(
                        &target,
                        ValidationType::Currency,
                        Some(&supported_currencies),
                    ),
                ];

                let results = join_all(validations).await;

                for result in results {
                    match result {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                }

                Ok(())
            }
            _ => Ok(()),
        };
    }

    async fn run(&self) {
        self.info_print();

        match self {
            // Action::InteractiveMode => interactive_mode::run().await,

            // Action::Help => {
            //     println!("Help");
            // }
            Action::Error { message } => {
                print_error(message.as_str());
            }
            Action::ListCurrencies => {
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
            Action::ListCurrenciesWithRates => {
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
            Action::DirectConversion {
                source,
                target,
                amount,
            } => {
                let rate = get_rate(source, target).await;
                let amount = amount.parse::<f64>().unwrap();

                match rate {
                    Ok(rate) => {
                        let conversion_results = convert(amount, rate);
                        println!(
                            "{} {} = {} {}  // exchange rate: {}",
                            amount.to_string().bold(),
                            source.to_uppercase(),
                            conversion_results.to_string().green(),
                            target.to_uppercase(),
                            rate.to_string().yellow()
                        );
                    }
                    Err(e) => {
                        println!("{}", e.red());
                    }
                }
            }
            Action::UpdateCache => {
                let reset_cache_folders = rest_cache().await;

                match reset_cache_folders {
                    Ok(_) => {
                        println!("Cache folders reset");
                    }
                    Err(e) => {
                        print_error(
                            format!("Failed to reset cache folders: {}", e.to_string()).as_str(),
                        );
                    }
                }

                let supported_currencies_with_rates = get_supported_currencies_with_rates().await;

                match supported_currencies_with_rates {
                    Ok(_) => {
                        println!("Cache updated");
                    }
                    Err(e) => {
                        print_error(format!("Failed to update cache: {}", e.to_string()).as_str());
                    }
                }
            }
            _ => {
                println!("Not implemented");
            }
        }
    }

    pub async fn execute(&self) {
        let validation = self.validate().await;

        match validation {
            Ok(_) => self.run().await,
            Err(e) => {
                self.info_print();
                println!("{}", e.red());
            }
        }
    }
}

pub async fn parse_cli_arguments(args: Vec<String>) -> Action {
    if args.len() == 2 {
        match args[1].as_str() {
            "listCurrencies" => return Action::ListCurrencies,
            "listCurrenciesWithRates" => return Action::ListCurrenciesWithRates,
            "updateCache" => return Action::UpdateCache,
            "help" => return Action::Help,
            _ => {
                return Action::Error {
                    message: "Invalid argument".to_string(),
                }
            }
        }
    }

    if args.len() == 4 {
        let source = args[1].clone();
        let target = args[2].clone();
        let amount = args[3].clone();

        return Action::DirectConversion {
            source,
            target,
            amount,
        };
    }

    return Action::Error {
        message: "No arguments provided".to_string(),
    };
}
