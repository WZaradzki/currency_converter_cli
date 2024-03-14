#[cfg(test)]
mod tests {
    use currency_converter_cli::{
        currency::Currency,
        validation::{validate, ValidationType},
    };

    #[tokio::test]
    async fn test_validation_currency_fail() {
        let accepted_currencies = vec![
            Currency::new_from_code("USD".to_string()),
            Currency::new_from_code("EUR".to_string()),
            Currency::new_from_code("JPY".to_string()),
        ];

        let validation = validate(
            &"test".to_string(),
            ValidationType::Currency,
            Some(&accepted_currencies),
        )
        .await;

        assert_eq!(validation, Err("TEST is not a valid currency".to_string()));
    }

    #[tokio::test]
    async fn test_validation_currency_success() {
        let accepted_currencies = vec![
            Currency::new_from_code("USD".to_string()),
            Currency::new_from_code("EUR".to_string()),
            Currency::new_from_code("JPY".to_string()),
        ];

        let validation = validate(
            &"USD".to_string(),
            ValidationType::Currency,
            Some(&accepted_currencies),
        )
        .await;

        assert_eq!(validation, Ok(()));
    }

    #[tokio::test]
    async fn test_validation_amount_fail() {
        let validation = validate(&"test".to_string(), ValidationType::Amount, None).await;

        assert_eq!(validation, Err("test is not a valid number".to_string()));
    }

    #[tokio::test]
    async fn test_validation_amount_success() {
        let validation = validate(&"100".to_string(), ValidationType::Amount, None).await;

        assert_eq!(validation, Ok(()));
    }
}
