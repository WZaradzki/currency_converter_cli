use colored::Colorize;
use futures::future::join_all;

use crate::validation::{validate, ValidationType};
pub mod interactive_mode;

#[derive(Debug, PartialEq)]
pub enum Action {
    InteractiveMode,
    DirectConversion {
        source: String,
        target: String,
        amount: f64,
    },
    UpdateCache,
    Help,
    Error {
        message: String,
    },
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
        }
    }

    pub fn run(&self) {
        self.info_print();
    }
}

pub async fn parse_cli_arguments(args: Vec<String>) -> Action {
    if args.len() == 1 {
        return Action::InteractiveMode;
    }

    if args.len() == 4 {
        let source = args[1].clone();
        let target = args[2].clone();
        let amount = args[3].clone();

        let validations = vec![
            validate(&amount, ValidationType::Amount),
            validate(&source, ValidationType::Currency),
            validate(&target, ValidationType::Currency),
        ];

        let results = join_all(validations).await;

        for result in results {
            match result {
                Ok(_) => (),
                Err(e) => return Action::Error { message: e },
            }
        }

        return Action::DirectConversion {
            source,
            target,
            amount: amount.parse::<f64>().unwrap(),
        };
    }

    return Action::Error {
        message: "No arguments provided".to_string(),
    };
}
