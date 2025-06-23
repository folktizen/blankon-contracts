use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin, space = 8 + BlankonState::LEN)]
    pub blankon_state: Account<'info, BlankonState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_handler(
    ctx: Context<Initialize>,
    gold_pyth_account: Pubkey,
    sol_pyth_account: Pubkey,
    fartcoin_pyth_account: Pubkey,
) -> Result<()> {
    let blankon_state = &mut ctx.accounts.blankon_state;
    blankon_state.admin = ctx.accounts.admin.key();

    blankon_state.markets[GOLD as usize] = MarketInfo {
        asset_type: GOLD,
        pyth_price_account: gold_pyth_account,
        skew: 0,
        total_long_size: 0,
        total_short_size: 0,
        last_funding_time: Clock::get()?.unix_timestamp,
        global_funding_index: 0,
    };

    blankon_state.markets[SOL as usize] = MarketInfo {
        asset_type: SOL,
        pyth_price_account: sol_pyth_account,
        skew: 0,
        total_long_size: 0,
        total_short_size: 0,
        last_funding_time: Clock::get()?.unix_timestamp,
        global_funding_index: 0,
    };

    blankon_state.markets[FARTCOIN as usize] = MarketInfo {
        asset_type: FARTCOIN,
        pyth_price_account: fartcoin_pyth_account,
        skew: 0,
        total_long_size: 0,
        total_short_size: 0,
        last_funding_time: Clock::get()?.unix_timestamp,
        global_funding_index: 0,
    };

    Ok(())
}
