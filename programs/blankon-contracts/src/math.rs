// Calculate price based on skew
pub fn calculate_price_from_skew(base_price: u64, skew: i64, skew_scale: u64) -> u64 {
    let skew_adjustment = if skew >= 0 {
        (skew as i128 * base_price as i128) / skew_scale as i128
    } else {
        -((skew.abs() as i128 * base_price as i128) / skew_scale as i128)
    };

    (base_price as i128 + skew_adjustment) as u64
}

// Calculate funding rate based on skew
pub fn calculate_funding_rate(skew: i64, skew_scale: u64, max_funding_rate: u64) -> i64 {
    ((skew as i128 * max_funding_rate as i128) / skew_scale as i128) as i64
}
