use crate::currency::Currency;

pub mod file_cache;

pub const CACHE_DIR: &str = "cache";
pub enum CacheConfigs {
    Currencies,
    ExchangeRates,
}

impl CacheConfigs {
    pub fn get_config(&self, currency: Option<Currency>) -> CacheConfig {
        match self {
            CacheConfigs::Currencies => CacheConfig::new(72, "currencies".to_string(), currency),
            CacheConfigs::ExchangeRates => {
                CacheConfig::new(1, "exchange_rates".to_string(), currency)
            }
        }
    }
}

pub struct CacheConfig {
    lifetime_in_hours: i32,
    dir_name: String,
    currency: Option<Currency>,
}
impl CacheConfig {
    pub fn new(
        lifetime_in_hours: i32,
        dir_name: String,
        currency: Option<Currency>,
    ) -> CacheConfig {
        CacheConfig {
            lifetime_in_hours,
            dir_name: CACHE_DIR.to_owned() + "/" + &dir_name,
            currency,
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
}
