use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::{
    Marketplace, DISCRIMINATOR, MARKETPLACE_SEED_PREFIX, REWARDS_SEED_PREFIX, TREASURY_SEED_PREFIX,
};

pub fn _initialize(ctx: &mut Context<InitializeAccounts>, input: &InitializeInput) -> Result<()> {
    ctx.accounts.marketplace.set_inner(Marketplace {
        admin: ctx.accounts.admin.key(),
        name: input.name.clone(),
        fee: input.fee,
        bump: ctx.bumps.marketplace,
        treasury_bump: ctx.bumps.treasury,
        reward_bump: ctx.bumps.rewards_mint,
    });
    Ok(())
}

#[derive(Accounts)]
#[instruction(input: InitializeInput)]
pub struct InitializeAccounts<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = DISCRIMINATOR + Marketplace::INIT_SPACE,
        seeds = [MARKETPLACE_SEED_PREFIX, input.name.as_bytes()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        seeds = [TREASURY_SEED_PREFIX, marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [REWARDS_SEED_PREFIX, marketplace.key().as_ref()],
        bump,
        mint::token_program = token_program,
        mint::decimals = 9,
        mint::authority = admin,
        mint::freeze_authority = admin
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeInput {
    pub name: String,
    pub fee: u16,
}
