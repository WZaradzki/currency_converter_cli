use assert_cmd::Command;

#[test]
fn test_validation_currency() {
    let mut cmd = Command::cargo_bin("currency_converter_cli").unwrap();
    cmd.arg("USD").arg("E2121UR").arg("10");

    cmd.assert().success().stdout("E2121UR is not a valid currency\n");
}
