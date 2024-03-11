pub mod interactive_mode;

pub enum Action {
    InteractiveMode,
    DirectConversion { source: String, target: String, amount: f64 },
    UpdateCache,
    Help,
    Error { message: String },
}

impl Action {
    pub fn info_print(&self) {
        match self {
            Action::InteractiveMode => println!("Running in interactive mode"),
            Action::DirectConversion { source, target, amount } => println!("Converting {} {} to {}", amount, source, target),
            Action::UpdateCache => println!("Updating cache"),
            Action::Help => println!("Printing help"),
            Action::Error { message } => println!("Error: {}", message),
        }
    }
}

pub fn parse_cli_arguments(args: Vec<String>) -> Action {
    // Logic to parse args and return an appropriate Action variant

    return Action::Error { message: "No arguments provided".to_string() };
}
