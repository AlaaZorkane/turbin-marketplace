use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Listing {
    pub maker: Pubkey,
    pub maker_mint: Pubkey,
    pub maker_mint_ata: Pubkey,
    pub price: u64,
    pub bump: u8,
    pub marketplace: Pubkey,
    pub marketplace_bump: u8,
}
