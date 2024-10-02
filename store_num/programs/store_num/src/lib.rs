use anchor_lang::prelude::*;

declare_id!("JKyzkyVHKFxJzQ9fm45T9PvSsYeQyfxhEREkHrcDqFM");

#[program]
pub mod store_num {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>, _num: u8) -> Result<()> {
        let store_num_account = &mut _ctx.accounts.store_number_account;
        store_num_account.num = _num;

        msg!("Number Stored : {}", _num);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, payer=signer, seeds=[b"Store_Number"], bump, space=8+1)]
    pub store_number_account: Account<'info, StoreNumber>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StoreNumber {
    num: u8,
}
