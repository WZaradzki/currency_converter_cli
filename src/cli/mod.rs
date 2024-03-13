use colored::Colorize;

use crate::error::print_error;

use self::{
    direct_conversion::{display_direct_conversion, validate_args},
    history::display_history,
    interactive_mode::{process_direct_conversion, start_interactive_mode},
    list_currencies::display_supported_currencies,
    list_currencies_with_rates::display_supported_currencies_with_rates,
    setup::setup_app,
    update_cache::update_cache,
};
pub mod direct_conversion;
pub mod history;
pub mod interactive_mode;
pub mod list_currencies;
pub mod list_currencies_with_rates;
pub mod setup;
pub mod update_cache;

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
    Setup,
    History,
}

impl Action {
    async fn from_interactive_mode(input: &str) {
        match input {
            "direct-conversion" => process_direct_conversion().await,
            "update-cache" => update_cache().await,
            "help" => Action::print_help_interactive_mode(),
            "list-currencies" => display_supported_currencies().await,
            "list-currencies-with-rates" => display_supported_currencies_with_rates().await,
            "setup" => setup_app().await,
            "history" => display_history().await,
            _ => print_error("Invalid command"),
        }
    }

    fn get_interactive_mode_actions() -> Vec<Action> {
        Action::get_all_actions()
            .into_iter()
            .filter(|action| match action {
                Action::InteractiveMode => false,
                _ => true,
            })
            .collect::<Vec<Action>>()
    }

    fn print_help_interactive_mode() {
        let actions = Action::get_interactive_mode_actions();
        for action in actions {
            action.into_help_print_interactive_mode();
        }
    }

    fn into_help_print_interactive_mode(&self) {
        match self {
            Action::DirectConversion { .. } => println!(
                "{} {} - {}",
                "direct-conversion".green(),
                "source target amount".green(),
                "Convert currency"
            ),
            Action::UpdateCache => println!("{}", "update-cache".green()),
            Action::Help => println!("{}", "help".green()),
            Action::ListCurrencies => println!("{}", "list-currencies".green()),
            Action::ListCurrenciesWithRates => println!("{}", "list-currencies-with-rates".green()),
            Action::Setup => println!("{}", "setup".green()),
            Action::History => println!("{}", "history".green()),
            _ => (),
        }
    }

    fn print_help() {
        let actions = Action::get_all_actions();
        for action in actions {
            action.into_help_print();
        }
    }

    fn get_all_actions() -> Vec<Action> {
        vec![
            Action::InteractiveMode,
            Action::UpdateCache,
            Action::Help,
            Action::ListCurrencies,
            Action::ListCurrenciesWithRates,
            Action::Setup,
            Action::History,
            Action::DirectConversion {
                source: "".to_string(),
                target: "".to_string(),
                amount: "".to_string(),
            },
        ]
    }

    fn new_from_single_argument(arg: &String) -> Action {
        match arg.as_str() {
            "-i" | "interactive" => Action::InteractiveMode,
            "-u" | "update-cache" => Action::UpdateCache,
            "-h" | "help" => Action::Help,
            "-l" | "list-currencies" => Action::ListCurrencies,
            "-lr" | "list-currencies-with-rates" => Action::ListCurrenciesWithRates,
            "-s" | "setup" => Action::Setup,
            "-H" | "history" => Action::History,
            _ => Action::Error {
                message: "Invalid argument".to_string(),
            },
        }
    }

    fn into_help_print(&self) {
        match self {
            Action::InteractiveMode => println!(
                "{} {} - {}",
                "-i".green(),
                "interactive".green(),
                "Run in interactive mode"
            ),
            Action::UpdateCache => println!(
                "{} {} - {}",
                "-u".green(),
                "update-cache".green(),
                "Update cache"
            ),
            Action::Help => println!("{} {} - {}", "-h".green(), "--help".green(), "Print help"),
            Action::ListCurrencies => println!(
                "{} {} - {}",
                "-l".green(),
                "list-currencies".green(),
                "List supported currencies"
            ),
            Action::ListCurrenciesWithRates => println!(
                "{} {} - {}",
                "-lr".green(),
                "list-currencies-with-rates".green(),
                "List supported currencies with rates"
            ),
            Action::Setup => println!(
                "{} {} - {}",
                "-s".green(),
                "setup".green(),
                "Setup application"
            ),
            Action::History => println!(
                "{} {} - {}",
                "-h".green(),
                "history".green(),
                "Display history"
            ),
            _ => (),
        }
    }

    fn new_direct_conversion(source: String, target: String, amount: String) -> Action {
        Action::DirectConversion {
            source,
            target,
            amount,
        }
    }

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
            } => validate_args(source, target, amount).await,
            _ => Ok(()),
        };
    }

    async fn run(&self) {
        self.info_print();

        match self {
            Action::Error { message } => print_error(message.as_str()),
            Action::ListCurrencies => display_supported_currencies().await,
            Action::ListCurrenciesWithRates => display_supported_currencies_with_rates().await,
            Action::DirectConversion {
                source,
                target,
                amount,
            } => display_direct_conversion(source, target, amount).await,
            Action::UpdateCache => update_cache().await,
            Action::Setup => setup_app().await,
            Action::History => display_history().await,
            Action::Help => Action::print_help(),
            Action::InteractiveMode => start_interactive_mode().await,
            _ => print_error("Not implemented yet"),
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
        return Action::new_from_single_argument(&args[1]);
    }

    if args.len() == 4 {
        return Action::new_direct_conversion(args[1].clone(), args[2].clone(), args[3].clone());
    }

    return Action::Error {
        message: "No arguments provided".to_string(),
    };
}

pub async fn missing_config() {
    println!(
        "{} {}",
        "No configuration found".red().bold(),
        "Running setup".yellow()
    );
    setup_app().await;
}
