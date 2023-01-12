use anchor_lang::prelude::*;

declare_id!("FHbBQDoHrMos9tKTMt9C5N2hqgM7Xhm9wRtkNLfFV1iB");

#[program]
pub mod myepicproject {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_selfies = 0;
        Ok(())
    }

    pub fn add_selfie(ctx: Context<AddSelfie>, selfie_link: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct {
            selfie_link: selfie_link.to_string(),
            user_address: *user.to_account_info().key,
        };

        base_account.selfie_list.push(item);
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
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddSelfie<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub selfie_link: String,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseAccount {
    pub total_selfies: u64,
    pub selfie_list: Vec<ItemStruct>,
}
