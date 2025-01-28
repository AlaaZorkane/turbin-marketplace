use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{Listing, Marketplace, DISCRIMINATOR, LISTING_SEED_PREFIX, MARKETPLACE_SEED_PREFIX};

pub fn _list(ctx: &mut Context<ListAccounts>, input: ListInput) -> Result<()> {
    ctx.accounts.listing.set_inner(Listing {
        bump: ctx.bumps.listing,
        marketplace: ctx.accounts.marketplace.key(),
        marketplace_bump: ctx.accounts.marketplace.bump,
        maker: ctx.accounts.maker.key(),
        maker_mint: ctx.accounts.maker_mint.key(),
        maker_mint_ata: ctx.accounts.maker_mint_ata.key(),
        price: input.price,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ListAccounts<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        mut,
        seeds = [MARKETPLACE_SEED_PREFIX, marketplace.name.as_bytes()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        init,
        payer = maker,
        space = DISCRIMINATOR + Listing::INIT_SPACE,
        seeds = [LISTING_SEED_PREFIX, marketplace.key().as_ref(), maker.key().as_ref()],
        bump
    )]
    pub listing: Account<'info, Listing>,
    pub maker_mint: InterfaceAccount<'info, Mint>,
    pub maker_mint_ata: InterfaceAccount<'info, TokenAccount>,
    pub metadata: Account<'info, MetadataAccount>,
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub collection_mint: InterfaceAccount<'info, Mint>,
    pub master_edition: Account<'info, MasterEditionAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ListInput {
    pub price: u64,
}
