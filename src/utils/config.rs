use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use colored::Colorize;

use crate::error::print_info;

const ENV_PATH: &str = ".env";
const ENV_VARS_WITH_DESCRIPTION: [(&str, &str, &str); 4] = [
    ("API_KEY",
    "String",
     "Get your API key from https://app.exchangerate-api.com/"),
    (
        "CURRENCY_CACHE_TIME_IN_HOURS",
        "u32",
        "Cache time to live in hours for currency list, if you don't want to cache rates set it to 0",
    ),
    (
        "CURRENCY_RATE_CACHE_TIME_IN_HOURS",
        "u32",
        "Cache time to live in hours for currency rates, if you don't want to cache rates set it to 0",
    ),
    (
        "COMMAND_HISTORY_CACHE_TIME_IN_HOURS",
        "u32",
        "Cache time to live in hours for command history, if you don't want to cache rates set it to 0",
    )
];

pub fn health_check() -> bool {
    if check_is_env_file_exist() && check_env_file_has_vars().is_ok_and(|x| x == true) {
        return true;
    }

    false
}

fn check_is_env_file_exist() -> bool {
    Path::new(ENV_PATH).exists()
}

pub fn remove_env_file() -> io::Result<()> {
    fs::remove_file(ENV_PATH)
}

fn check_env_file_has_vars() -> Result<bool, String> {
    let contents = fs::read_to_string(".env");

    if contents.is_err() {
        return Err("Failed to read .env file".to_string());
    }

    let contents = contents.unwrap();

    for (key, _, _) in ENV_VARS_WITH_DESCRIPTION {
        if !contents.contains(key) {
            return Ok(false);
        }
    }

    Ok(true)
}

pub fn check_config() -> Result<(), String> {
    if !check_is_env_file_exist() {
        print_info("No .env file found. Let's create one.");

        let file_creation = create_env_file(ENV_PATH);

        match file_creation {
            Ok(_) => {
                print_info(".env file created successfully");
                Ok(())
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    } else {
        let check_env_vars = check_env_file_has_vars();

        match check_env_vars {
            Ok(has_vars) => {
                if !has_vars {
                    print_info("Some environment variables are missing. Let's add them.");

                    let file_creation = create_env_file(ENV_PATH);

                    match file_creation {
                        Ok(_) => {
                            print_info("Environment variables added successfully");
                            Ok(())
                        }
                        Err(e) => {
                            return Err(e.to_string());
                        }
                    }
                } else {
                    Ok(())
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

fn create_env_file(path: &str) -> io::Result<()> {
    let mut file = File::create(path)?;

    for (key, value_type, desc) in ENV_VARS_WITH_DESCRIPTION {
        let prompt = desc.yellow();
        println!("{}", prompt);

        let write_var = write_env_var(key, value_type, &mut file);

        if write_var.is_err() {
            return Err(write_var.err().unwrap());
        }
    }

    Ok(())
}

fn write_env_var(key: &str, value_type: &str, file: &mut File) -> io::Result<()> {
    let mut var_value = String::new();
    let read_line = io::stdin().read_line(&mut var_value);

    if read_line.is_err() {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to read input"));
    } else {
        read_line.unwrap();
    }

    var_value = var_value.trim().to_string();

    if var_value.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("{} cannot be empty", key),
        ));
    }

    match check_type(&var_value, value_type) {
        Ok(_) => (),
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("{}: {}", key, e),
            ));
        }
    }

    let write: Result<(), io::Error> = writeln!(file, "{}={}", key, var_value);

    if write.is_err() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to write to .env file",
        ));
    }

    Ok(())
}

fn check_type(value: &String, value_type: &str) -> Result<(), String> {
    match value_type {
        "String" => Ok(()),
        "u32" => match value.parse::<u32>() {
            Ok(_) => Ok(()),
            Err(_) => Err("must be a number".to_string()),
        },
        _ => Err("Is not a valid type".to_string()),
    }
}
