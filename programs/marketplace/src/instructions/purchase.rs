use anchor_lang::prelude::*;

pub fn _purchase(_ctx: &Context<PurchaseAccounts>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct PurchaseAccounts<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
}
