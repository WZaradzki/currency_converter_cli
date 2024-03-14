#[cfg(test)]
mod tests {
    use currency_converter_cli::cli::{parse_cli_arguments, Action};

    #[tokio::test]
    async fn test_direct_conversion_parse_args() {
        let direct_conversion_args = vec![
            "currency_converter_cli".to_string(),
            "USD".to_string(),
            "EUR".to_string(),
            "100".to_string(),
        ];

        let action = parse_cli_arguments(direct_conversion_args).await;

        assert_eq!(
            action,
            Action::DirectConversion {
                source: "USD".to_string(),
                target: "EUR".to_string(),
                amount: "100".to_string()
            }
        );
    }

    #[tokio::test]
    async fn test_history_parse_args() {
        let history_args = vec!["currency_converter_cli".to_string(), "history".to_string()];

        let action = parse_cli_arguments(history_args).await;

        assert_eq!(action, Action::History);
    }

    #[tokio::test]
    async fn test_supported_currencies_parse_args() {
        let supported_currencies_args =
            vec!["currency_converter_cli".to_string(), "-l".to_string()];

        let action = parse_cli_arguments(supported_currencies_args).await;

        assert_eq!(action, Action::ListCurrencies);
    }

    #[tokio::test]
    async fn test_list_supported_currencies_with_rates_parse_args() {
        let supported_currencies_args =
            vec!["currency_converter_cli".to_string(), "-lr".to_string()];

        let action = parse_cli_arguments(supported_currencies_args).await;

        assert_eq!(action, Action::ListCurrenciesWithRates);
    }

    #[tokio::test]
    async fn test_invalid_args() {
        let invalid_args = vec!["currency_converter_cli".to_string()];

        let action = parse_cli_arguments(invalid_args).await;

        assert_eq!(
            action,
            Action::Error {
                message:
                    "Invalid number of arguments, use -h to see the list of available commands"
                        .to_string()
            }
        );
    }

    #[tokio::test]
    async fn test_help_parse_args() {
        let help_args = vec!["currency_converter_cli".to_string(), "-h".to_string()];

        let action = parse_cli_arguments(help_args).await;

        assert_eq!(action, Action::Help);
    }

    #[tokio::test]
    async fn test_setup_parse_args() {
        let setup_args = vec!["currency_converter_cli".to_string(), "setup".to_string()];

        let action = parse_cli_arguments(setup_args).await;

        assert_eq!(action, Action::Setup);
    }

    #[tokio::test]
    async fn test_update_cache_parse_args() {
        let update_cache_args = vec!["currency_converter_cli".to_string(), "-u".to_string()];

        let action = parse_cli_arguments(update_cache_args).await;

        assert_eq!(action, Action::UpdateCache);
    }

    #[tokio::test]
    async fn test_interactive_mode_parse_args() {
        let interactive_mode_args = vec!["currency_converter_cli".to_string(), "interactive".to_string()];

        let action = parse_cli_arguments(interactive_mode_args).await;

        assert_eq!(action, Action::InteractiveMode);
    }

    #[tokio::test]
    async fn test_error_parse_args() {
        let error_args = vec!["currency_converter_cli".to_string(), "invalid".to_string()];

        let action = parse_cli_arguments(error_args).await;

        assert_eq!(
            action,
            Action::Error {
                message: "Invalid argument".to_string()
            }
        );
    }
}
