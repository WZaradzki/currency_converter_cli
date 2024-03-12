use colored::Colorize;
use futures::future::join_all;

use crate::{
    api::get_supported_currencies, conversion::convert, currency::get_rate, error::print_error, validation::{validate, ValidationType}
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
}

impl Action {
    fn info_print(&self) {
        match self {
            Action::InteractiveMode => println!("Running in interactive mode"),
            Action::DirectConversion {
                source,
                target,
                amount,
            } => println!("Converting {} {} to {}", amount, source, target),
            Action::UpdateCache => println!("Updating cache"),
            Action::Help => println!("Printing help"),
            Action::Error { message } => println!("{} {}", "Error:".red().bold(), &message.red()),
            Action::ListCurrencies => println!("Listing supported currencies"),
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
                            "{} {} = {} {}  // rate: {}",
                            amount,
                            source,
                            conversion_results.to_string().green(),
                            target,
                            rate.to_string().yellow()
                        );
                    }
                    Err(e) => {
                        println!("{}", e.red());
                    }
                }
            }
            // Action::UpdateCache => {
            //     crate::api::update_cache().await;
            // }
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
