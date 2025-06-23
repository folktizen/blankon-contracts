pub mod constants;
pub mod errors;
pub mod instructions;
pub mod math;
pub mod pyth;
pub mod state;

pub use crate::instructions::*;

use anchor_lang::prelude::*;

declare_id!("99qG6bAkds5MpT37m61LFr8eJibvUGEu2GQ6TGxuwmda");

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

    pub fn open_position(
        ctx: Context<OpenPosition>,
        asset_type: u8,
        size: i64,
        leverage: u8,
    ) -> Result<()> {
        // Apply any pending funding before opening a new position
        // instructions::positions::apply_funding_handler(Context::new(
        //     &ctx.program_id,
        //     &ctx.accounts,
        //     &ctx.remaining_accounts,
        //     ctx.bumps.clone(),
        // ))?;

        open_handler(ctx, asset_type, size, leverage)
    }

    pub fn close_position(ctx: Context<ClosePosition>, asset_type: u8) -> Result<()> {
        // Apply any pending funding before closing the position
        // instructions::positions::apply_funding_handler(Context::new(
        //     &ctx.program_id,
        //     &ctx.accounts,
        //     &ctx.remaining_accounts,
        //     ctx.bumps.clone(),
        // ))?;

        close_handler(ctx, asset_type)
    }

    pub fn calculate_funding(ctx: Context<CalculateFunding>) -> Result<()> {
        calculate_funding_handler(ctx)
    }

    pub fn apply_funding(ctx: Context<ApplyFunding>) -> Result<()> {
        apply_funding_handler(ctx)
    }
}
