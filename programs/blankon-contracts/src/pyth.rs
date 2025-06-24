use crate::errors::*;
use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

pub const FEED_IDS: [&str; 3] = [
    "0x765d2ba906dbc32ca17cc11f5310a89e9ee1f6420508c63861f2f8ba4ee34bb2",
    "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d",
    "0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43",
];

pub fn get_pyth_price<'info>(
    pyth_price_account: &Account<'info, PriceUpdateV2>,
    market: u8,
) -> Result<u64> {
    let feed_id = FEED_IDS
        .get(market as usize)
        .ok_or(error!(DErrorCode::InvalidAssetType))?;
    let feed_id =
        get_feed_id_from_hex(feed_id).map_err(|_| error!(DErrorCode::InvalidOracleAccount))?;

    let price_update = pyth_price_account
        .get_price_unchecked(&feed_id)
        // .get_price_no_older_than(&Clock::get()?, 36000, &feed_id)
        .map_err(|_| error!(DErrorCode::InvalidOracleAccount))?;

    // Convert price to a standard format (e.g., USD with 6 decimals)
    // The price is represented as a fixed-point number with 'expo' number of decimal places
    // We want to convert it to a u64 with 6 decimal places

    // First, get the price as a signed integer
    let price_value = price_update.price;

    // Get the exponent (negative for decimal places)
    let expo = price_update.exponent;

    // Convert to a standard format with 6 decimal places
    let normalized_price = if expo <= -6 {
        // If expo is already more negative than -6, we need to divide
        // For example, if expo is -8, we divide by 10^(8-6) = 10^2
        let divisor = 10_i64.pow((expo.abs() - 6) as u32);
        (price_value / divisor) as u64
    } else {
        // If expo is less negative than -6 or positive, we need to multiply
        // For example, if expo is -4, we multiply by 10^(6-4) = 10^2
        let multiplier = 10_i64.pow((6 - expo.abs()) as u32);
        (price_value * multiplier) as u64
    };

    // Log the price information for debugging
    msg!(
        "Pyth price: raw={}, expo={}, normalized={}",
        price_value,
        expo,
        normalized_price
    );

    Ok(normalized_price as u64)
}
