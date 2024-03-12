pub mod converter;

pub fn convert(amount: f64, rate: f64) -> f64 {
    amount * rate
}