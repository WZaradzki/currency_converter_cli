use crate::{
    conversion::convert,
    currency::{get_rate, get_supported_currencies, history::CommandHistory},
    error::print_error,
    validation::{validate, ValidationType},
};
use colored::Colorize;
use futures::future::join_all;

pub async fn display_direct_conversion(source: &String, target: &String, amount: &str) {
    let rate = get_rate(source, target).await;
    let amount = amount.parse::<f64>().unwrap();

    match rate {
        Ok(rate) => {
            let conversion_results = convert(amount, rate);

            let output = format!(
                "{} {} = {} {}  // exchange rate: {}",
                amount.to_string().bold(),
                source.to_uppercase(),
                conversion_results.to_string().green(),
                target.to_uppercase(),
                rate.to_string().yellow()
            );

            println!("{}", output);

            let command = format!("{} {} {}", source, target, amount);
            let _ = CommandHistory::save(output, command);
        }
        Err(e) => {
            print_error(e.to_string().as_str());
        }
    }
}

pub async fn validate_args(
    source: &String,
    target: &String,
    amount: &String,
) -> Result<(), String> {
    let supported_currencies = get_supported_currencies().await;

    if supported_currencies.is_err() {
        return Ok(()); // we can't validate the currencies, validation will be transferred to api call
    }

    let supported_currencies = supported_currencies.unwrap();

    let validations = vec![
        validate(amount, ValidationType::Amount, None),
        validate(
            source,
            ValidationType::Currency,
            Some(&supported_currencies),
        ),
        validate(
            target,
            ValidationType::Currency,
            Some(&supported_currencies),
        ),
    ];

    let results = join_all(validations).await;

    for result in results {
        match result {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
