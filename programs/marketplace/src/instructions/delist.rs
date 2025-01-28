use anchor_lang::prelude::*;

pub fn _delist(_ctx: &Context<DelistAccounts>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct DelistAccounts<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
}
