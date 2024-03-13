# CLI Documentation for Currency Conversion Application

## Getting Started

This guide covers the initial setup for the currency conversion application.

### Prerequisites

- Ensure Rust and Cargo are installed on your system.

### Initial Configuration

- The application performs a health check at startup to verify the presence of an `.env` file with the necessary environment variables. If the `.env` file does not exist or lacks required variables, the application will prompt for setup. Alternatively, you can initiate setup manually with `-s` or `setup`.

### Environment Variables

- **API_KEY**: Obtain your API key from [Exchange Rate API](https://app.exchangerate-api.com/).
- **CURRENCY_CACHE_TIME_IN_HOURS**: Time To Live (TTL) for the currency list cache, in hours. Set to `0` to disable caching.
- **CURRENCY_RATE_CACHE_TIME_IN_HOURS**: TTL for currency rates cache, in hours. Set to `0` to disable caching.
- **COMMAND_HISTORY_CACHE_TIME_IN_HOURS**: TTL for command history cache, in hours. Set to `0` to disable caching.

## Usage

- Perform a direct conversion: `cargo run USD EUR 100`. Converts 100 USD to EUR at the current exchange rate.

## Commands

- **interactive-mode**: Start interactive mode.
- **help**: List all available commands and descriptions.
- **list-currencies**: Show all supported currencies.
- **list-currencies-with-rates**: Show all supported currencies with current exchange rates.
- **history**: Display the history of currency conversions. Useful for reviewing past activities.

## Config Commands

- **setup**: Initialize the application setup.
- **update-cache**: Update the cache with the latest currency exchange rates.

## Testing

### Prerequisites

- Ensure your `.env` file is correctly configured as per the "Getting Started" section.

### Running Tests

- Execute tests with: `cargo test`. This runs all automated tests to ensure application integrity.
