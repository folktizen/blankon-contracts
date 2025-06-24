use crate::constants::{
    INITIAL_MARGIN_REQUIREMENT, MAX_FUNDING_RATE, MAX_LEVERAGE, PERCENTAGE_DECIMALS,
    PRICE_DECIMALS, SKEW_SCALE,
};
use crate::math::calculate_funding_rate;
use crate::pyth::get_pyth_price;
use crate::state::*;
use crate::{errors::*, math::calculate_price_from_skew};
use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

#[derive(Accounts)]
pub struct UserStatus<'info> {
    pub blankon_state: Account<'info, BlankonState>,

    #[account(
        seeds = [b"user-account", user.key().as_ref()],
        bump,
        constraint = user_account.owner == user.key() @ DErrorCode::UnauthorizedAccess
    )]
    pub user_account: Account<'info, UserAccount>,

    pub user: Signer<'info>,

    pub pyth_price_account_gold: Account<'info, PriceUpdateV2>,
    pub pyth_price_account_sol: Account<'info, PriceUpdateV2>,
    pub pyth_price_account_btc: Account<'info, PriceUpdateV2>,

    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct MarketStatus<'info> {
    pub blankon_state: Account<'info, BlankonState>,

    pub pyth_price_account_gold: Account<'info, PriceUpdateV2>,
    pub pyth_price_account_sol: Account<'info, PriceUpdateV2>,
    pub pyth_price_account_btc: Account<'info, PriceUpdateV2>,

    pub clock: Sysvar<'info, Clock>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PositionStatus {
    pub size: i64,
    pub entry_price: u64,
    pub current_price_oracle: u64,
    pub current_price_amm: u64,
    pub unrealized_pnl: i64,
    pub initial_margin: u64,
    pub maintenance_margin: u64,
    pub claimable_value: i64,
    pub funding_index: i128,
    pub funding_rate: i64,
    pub last_funding_time: i64,
    pub leverage: u8,
}

fn get_position_status(
    blankon_state: &Account<'_, BlankonState>,
    user_account: &Account<'_, UserAccount>,
    pyth_price_account: &Account<'_, PriceUpdateV2>,
    asset_type: u8,
) -> Result<PositionStatus> {
    let position = user_account.positions[asset_type as usize];
    let market = blankon_state.markets[asset_type as usize];

    let oracle_price = get_pyth_price(pyth_price_account, asset_type)?;
    let amm_price = calculate_price_from_skew(oracle_price, market.skew, SKEW_SCALE);

    let position_size = position.size.abs() as u128 * position.leverage as u128;
    let entry_value = position_size * position.entry_price as u128 / PRICE_DECIMALS;
    let current_value = position_size * amm_price as u128 / PRICE_DECIMALS;

    let exists = position.size != 0 && position.leverage != 0;

    let initial_margin = if exists {
        entry_value as u64 * INITIAL_MARGIN_REQUIREMENT
            / PERCENTAGE_DECIMALS
            / position.leverage as u64
    } else {
        0
    };
    let maintenance_margin = current_value as u64 * INITIAL_MARGIN_REQUIREMENT
        / PERCENTAGE_DECIMALS
        / MAX_LEVERAGE as u64;

    let pnl = if position.size > 0 {
        // Long position: profit if exit_value > entry_value
        if current_value > entry_value {
            (current_value - entry_value) as i64
        } else {
            -((entry_value - current_value) as i64)
        }
    } else {
        // Short position: profit if entry_value > exit_value
        if entry_value > current_value {
            (entry_value - current_value) as i64
        } else {
            -((current_value - entry_value) as i64)
        }
    };

    let claimable_value = if pnl + initial_margin as i64 > 0 {
        pnl
    } else {
        0
    };

    let funding_rate = calculate_funding_rate(market.skew, SKEW_SCALE, MAX_FUNDING_RATE);

    Ok(PositionStatus {
        size: position.size * position.leverage as i64,
        entry_price: position.entry_price,
        current_price_oracle: oracle_price,
        current_price_amm: amm_price,
        unrealized_pnl: pnl,
        initial_margin,
        maintenance_margin,
        claimable_value,
        funding_index: market.global_funding_index,
        funding_rate,
        last_funding_time: market.last_funding_time,
        leverage: position.leverage,
    })
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MarketSnapshot {
    pub current_price_oracle: u64,
    pub current_price_amm: u64,
    pub funding_index: i128,
    pub funding_rate: i64,
    pub last_funding_time: i64,
}

fn get_market_status(
    blankon_state: &Account<'_, BlankonState>,
    pyth_price_account: &Account<'_, PriceUpdateV2>,
    asset_type: u8,
) -> Result<MarketSnapshot> {
    let market = blankon_state.markets[asset_type as usize];

    let oracle_price = get_pyth_price(pyth_price_account, asset_type)?;
    let amm_price = calculate_price_from_skew(oracle_price, market.skew, SKEW_SCALE);

    let funding_rate = calculate_funding_rate(market.skew, SKEW_SCALE, MAX_FUNDING_RATE);

    Ok(MarketSnapshot {
        current_price_oracle: oracle_price,
        current_price_amm: amm_price,
        funding_index: market.global_funding_index,
        funding_rate,
        last_funding_time: market.last_funding_time,
    })
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UserSnapshot {
    pub balance: u64,
    pub position_status: [PositionStatus; 3],
}

pub fn user_status_handler(ctx: Context<UserStatus>) -> Result<UserSnapshot> {
    let blankon_state = &ctx.accounts.blankon_state;
    let user_account = &ctx.accounts.user_account;

    let snapshot = UserSnapshot {
        balance: ctx.accounts.user_account.balance,
        position_status: [
            get_position_status(
                blankon_state,
                user_account,
                &ctx.accounts.pyth_price_account_gold,
                GOLD,
            )?,
            get_position_status(
                blankon_state,
                user_account,
                &ctx.accounts.pyth_price_account_sol,
                SOL,
            )?,
            get_position_status(
                blankon_state,
                user_account,
                &ctx.accounts.pyth_price_account_btc,
                BTC,
            )?,
        ],
    };

    Ok(snapshot)
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MarketSnapshots {
    pub market_snapshots: [MarketSnapshot; 3],
}

pub fn market_status_handler(ctx: Context<MarketStatus>) -> Result<MarketSnapshots> {
    let blankon_state = &ctx.accounts.blankon_state;

    Ok(MarketSnapshots {
        market_snapshots: [
            get_market_status(blankon_state, &ctx.accounts.pyth_price_account_gold, GOLD)?,
            get_market_status(blankon_state, &ctx.accounts.pyth_price_account_sol, SOL)?,
            get_market_status(blankon_state, &ctx.accounts.pyth_price_account_btc, BTC)?,
        ],
    })
}
