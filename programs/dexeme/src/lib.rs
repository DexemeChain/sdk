use anchor_lang::prelude::*;

declare_id!("Dexeme1111111111111111111111111111111111");

#[program]
pub mod dexeme {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let index = &mut ctx.accounts.index_account;
        index.total_deposits = 0;
        index.authority = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let index = &mut ctx.accounts.index_account;
        index.total_deposits += amount;
        Ok(())
    }
}

#[account]
pub struct IndexAccount {
    pub authority: Pubkey,
    pub total_deposits: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8)]
    pub index_account: Account<'info, IndexAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub index_account: Account<'info, IndexAccount>,
}
