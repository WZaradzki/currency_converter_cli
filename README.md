# CLI Documentation for Currency Conversion Application

## Getting Started

This guide covers the initial setup for the currency conversion application.


## Build

This section provides a simplified guide on how to build the currency conversion application from source.

#### Prerequisites

- Ensure Rust and Cargo are installed on your system.

#### Steps

1. **Clone the Repository**: Use the `git clone` command followed by the repository URL to clone the project to your local machine.
2. **Navigate to the Project Directory**: Change into the project's directory with the `cd` command.
3. **Build the Application**: Compile the application in release mode by running `cargo build --release`. This optimizes the application's performance and the output is located in `target/release`.
4. **Run the Application**: After building, you can run the application by executing the binary in the `target/release` directory. The binary name will match your project name.

**Updating the Application**:

- To update your application to the latest version, use the `git pull` command to fetch the latest changes and then re-run the build command to compile the latest version.


## Initial Configuration

- The application performs a health check at startup to verify the presence of an `.env` file with the necessary environment variables. If the `.env` file does not exist or lacks required variables, the application will prompt for setup. Alternatively, you can initiate setup manually with `-s` or `setup`.

```shell
target/release/currency_converter_cli -s
```

## Environment Variables

- **API_KEY**: Obtain your API key from [Exchange Rate API](https://app.exchangerate-api.com/).
- **CURRENCY_CACHE_TIME_IN_HOURS**: Time To Live (TTL) for the currency list cache, in hours. Set to `0` to disable caching.
- **CURRENCY_RATE_CACHE_TIME_IN_HOURS**: TTL for currency rates cache, in hours. Set to `0` to disable caching.
- **COMMAND_HISTORY_CACHE_TIME_IN_HOURS**: TTL for command history cache, in hours. Set to `0` to disable caching.

## Usage

- Three command-line arguments - `<source currency> <target currency> <amount>`
- Perform a direct conversion: `USD EUR 100`. Converts 100 USD to EUR at the current exchange rate. All supported currency codes at [Codes](https://www.exchangerate-api.com/docs/supported-currencies)

```shell
target/release/currency_converter_cli USD EUR 100
```

## Commands

- **interactive-mode**: Start interactive mode.
```shell
target/release/currency_converter_cli interactive-mode
target/release/currency_converter_cli -i
```
- **help**: List all available commands and descriptions.
```shell
target/release/currency_converter_cli help
target/release/currency_converter_cli -h
```
- **list-currencies**: Show all supported currencies.
```shell
target/release/currency_converter_cli list-currencies
target/release/currency_converter_cli -l
```
- **list-currencies-with-rates**: Show all supported currencies with current exchange rates.
```shell
target/release/currency_converter_cli list-currencies-with-rates
target/release/currency_converter_cli -lr
```
- **history**: Display the history of currency conversions. Useful for reviewing past activities.
```shell
target/release/currency_converter_cli history
target/release/currency_converter_cli -H
```


## Config Commands

- **setup**: Initialize the application setup.
```shell
target/release/currency_converter_cli setup
target/release/currency_converter_cli -s
```

- **update-cache**: Update the cache with the latest currency exchange rates.
```shell
target/release/currency_converter_cli update-cache
target/release/currency_converter_cli -u
```

## Testing

### Prerequisites

- Ensure your `.env` file is correctly configured as per the "Getting Started" section.

### Running Tests

- Execute tests with: `cargo test`. This runs all automated tests to ensure application integrity.
