use currency_converter_cli::{cli::parse_cli_arguments, utils::config::check_config};

#[tokio::main]
async fn main() -> () {
    if check_config() {
        parse_cli_arguments(std::env::args().collect()).await.run();
    } else {
        std::process::exit(1);
    }
}
