use crate::currency::Currency;
use chrono::Utc;

pub mod file_cache;

pub const CACHE_DIR: &str = "cache";

#[derive(Clone)]
pub enum CacheConfigs {
    Currencies,
    ExchangeRates,
    CommandHistory,
}

impl CacheConfigs {
    pub fn get_config(&self, currency: Option<Currency>) -> CacheConfig {
        // dotenv::dotenv().ok();

        match self {
            CacheConfigs::Currencies => {
                let now = Utc::now();
                let lifetime_in_hours = match std::env::var("CURRENCY_CACHE_TIME_IN_HOURS") {
                    Ok(val) => val.parse::<i32>().unwrap_or(24),
                    Err(_) => 24,
                };

                CacheConfig::new(
                    lifetime_in_hours,
                    "currencies".to_string(),
                    currency,
                    now.format("%Y-%m-%d_%H-%M-%S").to_string(),
                )
            }
            CacheConfigs::ExchangeRates => {
                let now = Utc::now();
                let lifetime_in_hours = match std::env::var("CURRENCY_RATE_CACHE_TIME_IN_HOURS") {
                    Ok(val) => val.parse::<i32>().unwrap_or(1),
                    Err(_) => 1,
                };

                CacheConfig::new(
                    lifetime_in_hours,
                    "exchange_rates".to_string(),
                    currency,
                    now.format("%Y-%m-%d_%H-%M-%S").to_string(),
                )
            }
            CacheConfigs::CommandHistory => {
                let lifetime_in_hours = match std::env::var("COMMAND_HISTORY_CACHE_TIME_IN_HOURS") {
                    Ok(val) => val.parse::<i32>().unwrap_or(1),
                    Err(_) => 1,
                };
                CacheConfig::new(
                    lifetime_in_hours,
                    "command_history".to_string(),
                    currency,
                    "commands".to_string(),
                )
            }
        }
    }
}

pub struct CacheConfig {
    lifetime_in_hours: i32,
    dir_name: String,
    currency: Option<Currency>,
    file_name: String,
}
impl CacheConfig {
    pub fn new(
        lifetime_in_hours: i32,
        dir_name: String,
        currency: Option<Currency>,
        file_name: String,
    ) -> CacheConfig {
        CacheConfig {
            lifetime_in_hours,
            dir_name: CACHE_DIR.to_owned() + "/" + &dir_name,
            currency,
            file_name,
        }
    }

    pub fn get_path(&self) -> String {
        let currency = self.currency.as_ref();
        if currency.is_none() {
            return self.dir_name.clone();
        }

        let currency = currency.unwrap();
        format!("{}/{}", self.dir_name, currency.get_code())
    }

    pub fn get_file_name(&self) -> String {
        self.file_name.clone()
    }

    pub fn is_cache_enabled(&self) -> bool {
        self.lifetime_in_hours > 0
    }
}
