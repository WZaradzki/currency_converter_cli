use chrono::{DateTime, Utc};

use serde::Serialize;
use serde_json::to_string_pretty;
use std::io::{self, Result};
use std::{fs, path::Path};

use crate::currency::Currency;

use super::{CacheConfigs, CACHE_DIR};

pub fn create_cache_file<T: Serialize>(
    serializable: &T,
    cache_config: CacheConfigs,
    currency: Option<Currency>,
) -> Result<()> {
    let config = cache_config.get_config(currency);
    if !config.is_cache_enabled() {
        return Err(io::Error::new(io::ErrorKind::Other, "Cache is not enabled"));
    }

    let json: String = to_string_pretty(serializable)?;

    let filename = format!("{}/{}.json", config.get_path(), config.get_file_name(),);

    let path = Path::new(&filename);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let write_file = fs::write(path, json);

    if write_file.is_err() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to write cache file",
        ));
    }

    Ok(())
}

pub fn read_and_invalid_cache_file<T: for<'de> serde::Deserialize<'de>>(
    cache_config: CacheConfigs,
    currency: Option<Currency>,
) -> Result<T> {
    let config = cache_config.get_config(currency);

    if !config.is_cache_enabled() {
        return Err(io::Error::new(io::ErrorKind::Other, "Cache is not enabled"));
    }

    let dir = config.get_path();
    let path = Path::new(dir.as_str());

    let entries = fs::read_dir(path);

    if entries.is_err() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No cache file found",
        ));
    }

    if let Some(entry) = entries.unwrap().next() {
        let entry_path = entry?.path();

        let created_time = entry_path.metadata();

        if created_time.is_err() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Cache file is expired",
            ));
        }

        let created_time = created_time.unwrap().created();

        if created_time.is_err() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Cache file is expired",
            ));
        }

        let now = Utc::now();
        let created_time: DateTime<Utc> = created_time.unwrap().into();
        let cache_lifetime = config.lifetime_in_hours as i64;
        let hours_parsed = chrono::Duration::try_hours(cache_lifetime);

        if hours_parsed.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Invalid cache lifetime",
            ));
        }

        let cache_time = now + hours_parsed.unwrap();

        if cache_time < created_time {
            fs::remove_file(entry_path).unwrap();
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Cache file is expired",
            ));
        }

        if entry_path.is_file() {
            let contents = fs::read_to_string(entry_path)?;
            let deserialized: T = serde_json::from_str(&contents)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

            return Ok(deserialized);
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "No cache file found",
    ))
}

pub async fn rest_cache() -> Result<()> {
    let cache_folder = fs::read_dir(CACHE_DIR);

    match cache_folder {
        Ok(_) => {
            let remove = fs::remove_dir_all(CACHE_DIR);

            match remove {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}
