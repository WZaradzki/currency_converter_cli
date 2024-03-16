use crate::{
    error::{print_error, print_info, print_warning},
    utils::config::{check_config, health_check, remove_env_file},
};

pub async fn setup_app() {
    if health_check() {
        print_info("Configuration is already set up, do you want to reset it? (y/n)");

        let mut input = String::new();

        let read = std::io::stdin().read_line(&mut input);

        if read.is_ok() {
            input = input.trim().to_string();
        } else {
            print_error("Failed to read input");
        }

        if input.to_lowercase().contains('y') {
            let reset = remove_env_file();

            if reset.is_err() {
                print_error("Failed to reset configuration");
            }
            
            match check_config() {
                Ok(_) => print_info("Configuration set up"),
                Err(e) => {
                    print_error(&e);
                }
            }
        } else {
            print_warning("Configuration not reset");
        }
    } else {
        match check_config() {
            Ok(_) => print_info("Configuration set up"),
            Err(e) => {
                print_error(&e);
            }
        }
    }
}
