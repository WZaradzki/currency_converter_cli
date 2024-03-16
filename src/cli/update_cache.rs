use crate::{
    cache::file_cache::rest_cache, currency::get_supported_currencies_with_rates,
    error::print_error,
};

pub async fn update_cache() {
    let reset_cache_folders = rest_cache().await;

    match reset_cache_folders {
        Ok(_) => {
            println!("Cache folders reset");
        }
        Err(e) => {
            print_error(format!("Failed to reset cache folders: {}", e).as_str());
        }
    }

    let supported_currencies_with_rates = get_supported_currencies_with_rates().await;

    match supported_currencies_with_rates {
        Ok(_) => {
            println!("Cache updated");
        }
        Err(e) => {
            print_error(e.to_string().as_str());
        }
    }
}
