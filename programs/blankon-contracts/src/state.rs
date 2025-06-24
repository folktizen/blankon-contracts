use anchor_lang::prelude::*;

#[account]
pub struct BlankonState {
    pub admin: Pubkey,
    pub markets: [MarketInfo; 3], // Gold, SOL, BTC
}

impl BlankonState {
    pub const LEN: usize = std::mem::size_of::<Self>();
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct MarketInfo {
    pub asset_type: u8,
    pub pyth_price_account: Pubkey,
    pub skew: i64,
    pub total_long_size: u64,
    pub total_short_size: u64,
    pub last_funding_time: i64,
    pub global_funding_index: i128, // Cumulative funding index
}

impl MarketInfo {
    pub const LEN: usize = std::mem::size_of::<Self>();
}

// Constants for asset types
pub const GOLD: u8 = 0;
pub const SOL: u8 = 1;
pub const BTC: u8 = 2;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct Position {
    pub size: i64,                // Positive for long, negative for short
    pub entry_price: u64,         // Price at entry
    pub leverage: u8,             // Leverage used for the position
    pub last_funding_index: i128, // Last funding index applied to this position
}

#[account]
pub struct UserAccount {
    pub owner: Pubkey,
    pub balance: u64,             // $10,000 in lamports equivalent
    pub positions: [Position; 3], // Gold, SOL, BTC positions
}

impl UserAccount {
    pub const LEN: usize = std::mem::size_of::<Self>();
}
