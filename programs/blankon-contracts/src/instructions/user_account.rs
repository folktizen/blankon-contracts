use crate::constants::*;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateUserAccount<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + // discriminator
               32 + // owner pubkey
               8 +  // balance (u64)
               3 * (8 + 8 + 8), // 3 positions (size, entry_price, last_funding_index)
        seeds = [b"user-account", user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_handler(ctx: Context<CreateUserAccount>) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;

    // Set the owner
    user_account.owner = ctx.accounts.user.key();

    // Initialize with $10,000 balance (in lamports, assuming 6 decimal places)
    user_account.balance = INITIAL_BALANCE; // 10_000_000_000 (10,000 with 6 decimals)

    // Initialize empty positions for all three markets
    for i in 0..3 {
        user_account.positions[i] = Position {
            size: 0,
            entry_price: 0,
            last_funding_index: 0,
        };
    }

    msg!(
        "User account created with initial balance of {} units",
        INITIAL_BALANCE
    );

    Ok(())
}
