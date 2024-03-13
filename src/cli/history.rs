use crate::{currency::history::get_history, error::print_error};

pub async fn display_history() {
    let history_commands = get_history().await;
    match history_commands {
        Ok(history_commands) => {
            for command in history_commands {
                println!(
                    "{}: {} // time: {}",
                    command.get_command(),
                    command.get_output(),
                    command.get_timestamp()
                );
            }
        }
        Err(e) => {
            print_error(e.as_str());
        }
    }
}
