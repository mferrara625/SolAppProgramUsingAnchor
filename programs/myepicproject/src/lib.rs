use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod myepicproject {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        let base_account = &mut ctx.accounts.base_account;
        base_account.total_selfies = 0;
        Ok(())
    }

    pub fn add_selfie(ctx: Context<AddSelfie>) -> Result <()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_selfies += 1;
        Ok(())
      }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct AddSelfie<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub total_selfies: u64
}
