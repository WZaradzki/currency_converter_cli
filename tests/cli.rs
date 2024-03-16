use assert_cmd::Command;
use predicates::str::{contains, starts_with};

#[test]
fn test_validation_currency() {
    let mut cmd = Command::cargo_bin("currency_converter_cli").unwrap();
    cmd.arg("USD").arg("E2121UR").arg("10");

    cmd.assert()
        .success()
        .stdout("E2121UR is not a valid currency\n");
}

#[test]
fn test_validation_amount() {
    let mut cmd = Command::cargo_bin("currency_converter_cli").unwrap();
    cmd.arg("USD").arg("EUR").arg("aa");

    cmd.assert().success().stdout("aa is not a valid number\n");
}

#[test]
fn test_validation_currency_and_amount() {
    let mut cmd = Command::cargo_bin("currency_converter_cli").unwrap();
    cmd.arg("-h");

    let predicate = starts_with("Default use - Direct conversion");
    cmd.assert().success().stdout(predicate);
}

#[test]
fn test_direct_conversion() {
    let mut cmd = Command::cargo_bin("currency_converter_cli").unwrap();
    cmd.arg("PLN").arg("USD").arg("10");

    let predicate_target_currency_with_amount = contains("10 PLN =");
    let exchange_rate = contains("exchange rate:");
    cmd.assert().success().stdout(predicate_target_currency_with_amount);
    cmd.assert().success().stdout(exchange_rate);
}

#[test]
fn test_list_currencies() {
    let mut cmd = Command::cargo_bin("currency_converter_cli").unwrap();
    cmd.arg("-l");

    let pln_currency = contains("PLN");
    let usd_currency = contains("USD");
    cmd.assert().success().stdout(pln_currency);
    cmd.assert().success().stdout(usd_currency);
}

