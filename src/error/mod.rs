use colored::Colorize;

pub fn print_error(message: &str) {
    println!("{} {}", "Error:".red(), message.red());
}

pub fn print_warning(message: &str) {
    println!("{} {}", "Warning:".yellow(), message.yellow());
}
