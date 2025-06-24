use crate::constants::*;
use crate::errors::*;
use crate::math::*;
use crate::pyth::*;
use crate::state::*;
use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub blankon_state: Account<'info, BlankonState>,

    #[account(
        mut,
        seeds = [b"user-account", user.key().as_ref()],
        bump,
        constraint = user_account.owner == user.key() @ DErrorCode::UnauthorizedAccess
    )]
    pub user_account: Account<'info, UserAccount>,

    pub user: Signer<'info>,

    pub pyth_price_account: Account<'info, PriceUpdateV2>,
}

pub fn apply_funding_handler(ctx: &mut Context<OpenPosition>) -> Result<()> {
    let blankon_state = &ctx.accounts.blankon_state;

    // Process each market
    for (asset_idx, market) in blankon_state.markets.iter().enumerate() {
        let position = ctx.accounts.user_account.positions[asset_idx];

        // Skip if user has no position
        if position.size == 0 {
            continue;
        }

        // Calculate funding based on the difference between global and user's last funding index
        let funding_index_delta = market.global_funding_index - position.last_funding_index as i128;

        // Skip if no funding to apply
        if funding_index_delta == 0 {
            continue;
        }

        // Calculate position notional value
        let position_notional =
            (position.size.abs() as u128 * position.entry_price as u128 / PRICE_DECIMALS) as u64;

        // Calculate funding amount
        // Funding amount = position notional * funding index delta
        let funding_amount =
            (position_notional as i128 * funding_index_delta) / FUNDING_INDEX_DECIMALS as i128;

        // Apply funding:
        // - Long positions pay positive funding rate (pay when funding_index_delta > 0)
        // - Short positions pay negative funding rate (pay when funding_index_delta < 0)
        let funding_to_pay = if position.size > 0 {
            // Long position
            funding_amount
        } else {
            // Short position
            -funding_amount
        };

        // Update user balance based on funding
        if funding_to_pay > 0 {
            // User pays funding
            let funding_payment = funding_to_pay as u64;

            // Cap the payment to the user's balance to prevent underflow

            {
                let user_account = &mut ctx.accounts.user_account;

                let payment = std::cmp::min(funding_payment, user_account.balance);
                user_account.balance = user_account
                    .balance
                    .checked_sub(payment)
                    .ok_or(DErrorCode::MathOverflow)?;

                msg!(
                    "User paid {} funding for {} position in market {}",
                    payment,
                    if position.size > 0 { "LONG" } else { "SHORT" },
                    asset_idx
                );
            }
        } else if funding_to_pay < 0 {
            // User receives funding
            let funding_receipt = (-funding_to_pay) as u64;

            {
                let user_account = &mut ctx.accounts.user_account;
                user_account.balance = user_account
                    .balance
                    .checked_add(funding_receipt)
                    .ok_or(DErrorCode::MathOverflow)?;
            }

            msg!(
                "User received {} funding for {} position in market {}",
                funding_receipt,
                if position.size > 0 { "LONG" } else { "SHORT" },
                asset_idx
            );
        }

        // Update position's funding index to match global index
        {
            let position = &mut ctx.accounts.user_account.positions[asset_idx];
            position.last_funding_index = market.global_funding_index;
        }
    }

    Ok(())
}

// ===== OPEN POSITION =====

pub fn open_handler(
    ctx: Context<OpenPosition>,
    asset_type: u8,
    size: i64,
    leverage: u8,
) -> Result<()> {
    // Validate inputs
    require!(asset_type < 3, DErrorCode::InvalidAssetType);
    require!(size != 0, DErrorCode::InvalidPositionSize);
    require!(
        leverage > 0 && leverage <= MAX_LEVERAGE,
        DErrorCode::InvalidLeverage
    );

    let blankon_state = &mut ctx.accounts.blankon_state;

    // Get the market info
    let market = &mut blankon_state.markets[asset_type as usize];

    // Verify the correct Pyth account is provided
    require!(
        market.pyth_price_account == ctx.accounts.pyth_price_account.key(),
        DErrorCode::InvalidOracleAccount
    );

    // Update market skew
    if size > 0 {
        // Long position
        market.total_long_size = market
            .total_long_size
            .checked_add(size as u64)
            .ok_or(DErrorCode::MathOverflow)?;
    } else {
        // Short position
        market.total_short_size = market
            .total_short_size
            .checked_add((-size) as u64)
            .ok_or(DErrorCode::MathOverflow)?;
    }

    // Recalculate market skew
    market.skew = market.total_long_size as i64 - market.total_short_size as i64;

    // Get the current price from Pyth
    let base_price = get_pyth_price(&ctx.accounts.pyth_price_account, asset_type)?;

    // Calculate the entry price based on market skew
    let entry_price = calculate_price_from_skew(base_price, market.skew, SKEW_SCALE);

    // Calculate the required margin
    let position_notional =
        (size.abs() as u128 * leverage as u128 * entry_price as u128 / PRICE_DECIMALS) as u64;
    let required_margin =
        ((position_notional * INITIAL_MARGIN_REQUIREMENT) / PERCENTAGE_DECIMALS) / leverage as u64;

    require!(
        ctx.accounts.user_account.balance >= required_margin,
        DErrorCode::InsufficientBalance
    );

    // Check if user already has a position for this asset
    {
        let position = &mut ctx.accounts.user_account.positions[asset_type as usize];
        require!(position.size == 0, DErrorCode::PositionAlreadyExists);
    }

    let user_account = &mut ctx.accounts.user_account;

    // Update user's balance
    user_account.balance = user_account
        .balance
        .checked_sub(required_margin)
        .ok_or(DErrorCode::MathOverflow)?;

    {
        let position = &mut ctx.accounts.user_account.positions[asset_type as usize];
        // Update position
        position.size = size;
        position.entry_price = entry_price;
        position.last_funding_index = 0; // Will be updated in funding calculations
        position.leverage = leverage;
    }

    msg!(
        "Opened {} position for asset {}: size={}, leverage={}x, margin={}, entry_price={}",
        if size > 0 { "LONG" } else { "SHORT" },
        asset_type,
        size.abs(),
        leverage,
        required_margin,
        entry_price
    );

    Ok(())
}

// ===== CLOSE POSITION =====

pub fn close_handler(ctx: Context<OpenPosition>, asset_type: u8) -> Result<()> {
    // Validate inputs
    require!(asset_type < 3, DErrorCode::InvalidAssetType);

    let blankon_state = &mut ctx.accounts.blankon_state;

    // Get the market info
    let market = &mut blankon_state.markets[asset_type as usize];

    // Verify the correct Pyth account is provided
    require!(
        market.pyth_price_account == ctx.accounts.pyth_price_account.key(),
        DErrorCode::InvalidOracleAccount
    );

    // Get the position
    let position = ctx.accounts.user_account.positions[asset_type as usize];

    // Check if position exists
    require!(position.size != 0, DErrorCode::NoPositionExists);

    // Get the current price from Pyth
    let base_price = get_pyth_price(&ctx.accounts.pyth_price_account, asset_type)?;

    // Calculate the exit price based on market skew
    let exit_price = calculate_price_from_skew(base_price, market.skew, SKEW_SCALE);

    // Calculate PnL
    let position_size = position.size.abs() as u128 * position.leverage as u128;
    let entry_value = position_size * position.entry_price as u128 / PRICE_DECIMALS;
    let exit_value = position_size * exit_price as u128 / PRICE_DECIMALS;

    let pnl = if position.size > 0 {
        // Long position: profit if exit_value > entry_value
        if exit_value > entry_value {
            (exit_value - entry_value) as i64
        } else {
            -((entry_value - exit_value) as i64)
        }
    } else {
        // Short position: profit if entry_value > exit_value
        if entry_value > exit_value {
            (entry_value - exit_value) as i64
        } else {
            -((exit_value - entry_value) as i64)
        }
    };

    // Calculate the margin that was locked
    let position_notional =
        (position.size.abs() as u128 * position.leverage as u128 * position.entry_price as u128
            / PRICE_DECIMALS) as u64;
    let locked_margin = position_notional * INITIAL_MARGIN_REQUIREMENT
        / PERCENTAGE_DECIMALS
        / position.leverage as u64;

    let user_account: &mut Account<'_, UserAccount> = &mut ctx.accounts.user_account;

    // Update user's balance (return margin + PnL)
    if pnl >= 0 {
        let new_balance = user_account
            .balance
            .checked_add(locked_margin)
            .ok_or(DErrorCode::MathOverflow)?
            .checked_add(pnl as u64)
            .ok_or(DErrorCode::MathOverflow)?;
        user_account.balance = new_balance;
    } else {
        // Ensure we don't underflow if loss exceeds margin
        let loss = pnl.abs() as u64;
        if loss >= locked_margin {
            // Loss exceeds margin, user loses entire margin
            // No need to subtract from balance as margin was already deducted when opening
        } else {
            // Return remaining margin after loss
            user_account.balance = user_account
                .balance
                .checked_add(locked_margin - loss)
                .ok_or(DErrorCode::MathOverflow)?;
        }
    }

    // Update market skew
    if position.size > 0 {
        // Long position
        market.total_long_size = market
            .total_long_size
            .checked_sub(position.size as u64)
            .ok_or(DErrorCode::MathOverflow)?;
    } else {
        // Short position
        market.total_short_size = market
            .total_short_size
            .checked_sub((-position.size) as u64)
            .ok_or(DErrorCode::MathOverflow)?;
    }

    // Recalculate market skew
    market.skew = market.total_long_size as i64 - market.total_short_size as i64;

    {
        let position = &mut ctx.accounts.user_account.positions[asset_type as usize];
        // Clear the position
        position.size = 0;
        position.entry_price = 0;
        position.last_funding_index = 0;
        position.leverage = 0;
    }

    msg!(
        "Closed position for asset {}: PnL={}, exit_price={}",
        asset_type,
        pnl,
        exit_price
    );

    Ok(())
}
