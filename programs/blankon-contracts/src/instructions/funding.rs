use crate::math::*;
use crate::state::*;
use crate::constants::*;
use crate::errors::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CalculateFunding<'info> {
    #[account(mut)]
    pub blankon_state: Account<'info, BlankonState>,

    // Optional: If you want to restrict this to admin only
    #[account(constraint = admin.key() == blankon_state.admin @ DErrorCode::UnauthorizedAccess)]
    pub admin: Signer<'info>,

    // System clock to check time elapsed
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<CalculateFunding>) -> Result<()> {
    let blankon_state = &mut ctx.accounts.blankon_state;
    let current_time = ctx.accounts.clock.unix_timestamp;

    // Process each market
    for (asset_idx, market) in blankon_state.markets.iter_mut().enumerate() {
        // Check if enough time has passed since last funding (1 hour = 3600 seconds)
        let time_elapsed = current_time - market.last_funding_time;
        if time_elapsed < FUNDING_INTERVAL {
            msg!(
                "Skipping funding for market {}: not enough time elapsed",
                asset_idx
            );
            continue;
        }

        // Calculate funding rate based on market skew
        let funding_rate = calculate_funding_rate(market.skew, SKEW_SCALE, MAX_FUNDING_RATE);

        // Skip if funding rate is zero
        if funding_rate == 0 {
            msg!(
                "Skipping funding for market {}: funding rate is zero",
                asset_idx
            );
            continue;
        }

        // Calculate funding index increment
        // For hourly funding, time factor is 1/24 (assuming 24 hours in a day)
        let time_factor = time_elapsed as u64 * TIME_FACTOR_DECIMALS / SECONDS_IN_DAY;

        // Funding index increment = funding rate * time factor
        let funding_index_increment = (funding_rate as i128 * time_factor as i128)
            / (PERCENTAGE_DECIMALS as i128 * TIME_FACTOR_DECIMALS as i128);

        // Update global funding index
        market.global_funding_index = market
            .global_funding_index
            .checked_add(funding_index_increment)
            .ok_or(DErrorCode::MathOverflow)?;

        // Update market's last funding time
        market.last_funding_time = current_time;

        msg!(
            "Global funding updated for market {}: rate={}, index_increment={}, new_index={}",
            asset_idx,
            funding_rate,
            funding_index_increment,
            market.global_funding_index
        );
    }

    Ok(())
}
