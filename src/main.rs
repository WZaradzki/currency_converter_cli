use currency_converter_cli::{
    cli::{missing_config, parse_cli_arguments},
    utils::config::health_check,
};

#[tokio::main]
async fn main() -> () {
    if health_check() {
        let action = parse_cli_arguments(std::env::args().collect()).await;

        action.execute().await;
    } else {
        missing_config().await;
    }
}
