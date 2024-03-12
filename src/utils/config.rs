use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use colored::Colorize;

use crate::error::print_error;

pub fn check_config() -> bool {
    let env_path = ".env";
    if !Path::new(env_path).exists() {
        println!("{}", "No .env file found. Let's create one.".red());
        let file_creation = create_env_file(env_path);
        match file_creation {
            Ok(_) => {
                println!("{}", ".env file created successfully".green());
                true
            }
            Err(e) => {
                print_error(e.to_string().as_str());
                return false;
            }
        }
    } else {
        // check it has the API_KEY
        let contents = fs::read_to_string(env_path).expect("Failed to read .env file");
        if !contents.contains("API_KEY") {
            println!("{}", "No API_KEY found in .env file. Let's add it.".red());
            let config_update = create_env_file(env_path);

            match config_update {
                Ok(_) => {
                    println!("{}", "API KEY added successfully".green());
                    return true
                }
                Err(e) => {
                    print_error(e.to_string().as_str());
                    return false;
                }
            }
        
        }

        true
    }
}

fn create_env_file(path: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    let api_key_prompt = "Please enter your API key: ".yellow();
    println!("{}", api_key_prompt);

    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key)?;
    api_key = api_key.trim().to_string(); // Trim newline and whitespace

    if api_key.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "API key cannot be empty",
        ));
    }

    let write = writeln!(file, "API_KEY={}", api_key);

    if write.is_err() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to write to .env file",
        ));
    }

    Ok(())
}

