use currency_converter_cli::{cli::parse_cli_arguments, utils::config::check_config};

#[tokio::main]
async fn main() -> () {
    if check_config() {
        let action = parse_cli_arguments(std::env::args().collect()).await;
        
        action.execute().await;
    } else {
        std::process::exit(1);
    }
}
