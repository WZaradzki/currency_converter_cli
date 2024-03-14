#[cfg(test)]
mod tests {
    use currency_converter_cli::{
        api::{currencies::get_supported_currencies_from_api, exchange_rate::get_exchange_rates},
        currency::Currency,
    };
    use tokio;

    #[tokio::test]
    async fn test_supported_currencies() {
        dotenv::dotenv().ok();
        let result = get_supported_currencies_from_api().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_currency_rates() {
        dotenv::dotenv().ok();
        let usd_currency = Currency::new_from_code("USD".to_string());
        let result = get_exchange_rates(usd_currency).await;

        assert!(result.is_ok());
    }
}
