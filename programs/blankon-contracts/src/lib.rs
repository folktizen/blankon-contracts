pub mod constants;
pub mod errors;
pub mod instructions;
pub mod math;
pub mod pyth;
pub mod state;

pub use crate::instructions::*;

use anchor_lang::prelude::*;
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    // Required fields
    name: "BLANKON",
    project_url: "https://blankon.folktizen.xyz",
    contacts: "email:security@folktizen.xyz",
    policy: "https://github.com/folktizen/blankon-contracts/blob/main/SECURITY.md",

    // Optional Fields
    preferred_languages: "en,id",
    source_code: "https://github.com/folktizen/blankon-contracts",
    auditors: "None yet!",
    acknowledgements: "
Shoutout to the contributors and whitehats who keep BLANKON safe!
If you find a bug, you’re a hero.
- The Folktizen Team
"
}

declare_id!("AA9xjMbf543L5vHqTceDGsFFKRW1ZXdTC6T8f33ux6yf");

#[program]
pub mod blankon_contracts {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        gold_pyth_account: Pubkey,
        sol_pyth_account: Pubkey,
        btc_pyth_account: Pubkey,
    ) -> Result<()> {
        initialize_handler(ctx, gold_pyth_account, sol_pyth_account, btc_pyth_account)
    }

    pub fn create_user_account(ctx: Context<CreateUserAccount>) -> Result<()> {
        create_handler(ctx)
    }

    pub fn open_position<'info>(
        mut ctx: Context<'_, '_, '_, 'info, OpenPosition<'info>>,
        asset_type: u8,
        size: i64,
        leverage: u8,
    ) -> Result<()> {
        // Apply any pending funding before opening a new position
        apply_funding_handler(&mut ctx)?;
        open_handler(ctx, asset_type, size, leverage)
    }

    pub fn close_position(mut ctx: Context<OpenPosition>, asset_type: u8) -> Result<()> {
        // Apply any pending funding before closing the position
        apply_funding_handler(&mut ctx)?;
        close_handler(ctx, asset_type)
    }

    pub fn calculate_funding(ctx: Context<CalculateFunding>) -> Result<()> {
        calculate_funding_handler(ctx)
    }

    pub fn apply_funding(mut ctx: Context<OpenPosition>) -> Result<()> {
        apply_funding_handler(&mut ctx)
    }

    pub fn get_user_status(ctx: Context<UserStatus>) -> Result<UserSnapshot> {
        user_status_handler(ctx)
    }

    pub fn get_market_status(ctx: Context<MarketStatus>) -> Result<MarketSnapshots> {
        market_status_handler(ctx)
    }
}
