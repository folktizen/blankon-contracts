// Initial balance for new users ($10,000 with 6 decimal places)
pub const INITIAL_BALANCE: u64 = 10_000_000_000;

// Initial and maintenance margin requirements (as percentages with 4 decimal places)
pub const INITIAL_MARGIN_REQUIREMENT: u64 = 1_000; // 10% (0.1 * 10000)
pub const MAINTENANCE_MARGIN_REQUIREMENT: u64 = 500; // 5% (0.05 * 10000)

// Maximum leverage allowed
pub const MAX_LEVERAGE: u8 = 10;

// Funding index precision
pub const FUNDING_INDEX_DECIMALS: u64 = 1_000_000_000_000; // 12 decimal places

// Skew scale for price impact calculations
pub const SKEW_SCALE: u64 = 1_000_000_000; // 1 billion

// Decimal precision for prices (6 decimals)
pub const PRICE_DECIMALS: u128 = 1_000_000;

// Decimal precision for percentages (4 decimals)
pub const PERCENTAGE_DECIMALS: u64 = 10_000;

// Funding related constants
pub const FUNDING_INTERVAL: i64 = 3600; // 1 hour in seconds
pub const SECONDS_IN_DAY: u64 = 86400; // 24 hours in seconds
pub const MAX_FUNDING_RATE: u64 = 100; // 1% with 4 decimal places (0.0001 * 10000)
pub const TIME_FACTOR_DECIMALS: u64 = 1_000_000; // 6 decimal places for time factor precision
