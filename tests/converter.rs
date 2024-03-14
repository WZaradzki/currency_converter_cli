#[cfg(test)]
mod tests {
    use currency_converter_cli::conversion::convert;

    #[tokio::test]
    async fn test_converter() {
        let amount = 100.0;
        let rate = 2.0;

        let conversion = convert(amount, rate);

        assert_eq!(conversion, 200.0);
    }
}
