use std::io;

use crate::{
    cli::Action,
    error::{print_error, print_info},
};

use super::direct_conversion::{display_direct_conversion, validate_args};

pub async fn start_interactive_mode() {
    print_info("Welcome to the interactive mode! You can type 'help' to see the list of available commands.");

    let mut input = String::new();
    loop {
        println!("Enter a command or 'exit' to quit // Type 'help' to see the list of available commands:");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "exit" {
            return print_info("Goodbye!");
        }

        Action::from_interactive_mode(input).await;
    }
}

pub async fn process_direct_conversion() {
    print_info("Welcome to the direct conversion mode!");

    let input_desc = vec![
        "Enter the source currency code:",
        "Enter the target currency code:",
        "Enter the amount to convert:",
    ];
    let mut inputs: Vec<String> = vec![];

    for desc in input_desc {
        println!("{}", desc);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        inputs.push(input.to_string());
    }

    let validate_inputs = validate_args(&inputs[0], &inputs[1], &inputs[2]).await;

    match validate_inputs {
        Ok(_) => (),
        Err(e) => {
            print_error(e.as_str());
            return;
        }
    }

    display_direct_conversion(&inputs[0], &inputs[1], &inputs[2]).await;
}
