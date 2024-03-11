use colored::Colorize;

pub fn print_error(message: &str) {
    println!("{} {}", "Error:".red(), message.red());
}
