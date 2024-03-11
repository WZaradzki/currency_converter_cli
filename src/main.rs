use currency_converter_cli::cli::{parse_cli_arguments, Action};


fn main() {
    let action = parse_cli_arguments(std::env::args().collect());

    match action {
        Action::InteractiveMode => cli::run_interactive_mode(),
        Action::DirectConversion { source, target, amount } => cli::perform_direct_conversion(&source, &target, amount),
        Action::UpdateCache => cli::update_cache(),
        Action::Help => cli::print_help(),
        Action::Error { message } => {
            eprintln!("{}", message);
            cli::print_help();
            std::process::exit(1);
        },
    }
}