use anchor_lang::prelude::*;

#[account]
pub struct BlankonState {
    pub admin: Pubkey,
    pub markets: [MarketInfo; 3], // gold, SOL, fartcoin
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
pub const FARTCOIN: u8 = 2;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct Position {
    pub size: i64,                // Positive for long, negative for short
    pub entry_price: u64,         // Price at entry
    pub last_funding_index: i128, // Last funding index applied to this position
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PositionStatus {
    pub size: i64,
    pub entry_price: u64,
    pub current_price: u64,
    pub unrealized_pnl: i64,
    pub initial_margin: u64,
    pub maintenance_margin: u64,
    pub claimable_value: u64,
}

#[account]
pub struct UserAccount {
    pub owner: Pubkey,
    pub balance: u64,             // $10,000 in lamports equivalent
    pub positions: [Position; 3], // gold, SOL, fartcoin positions
}
