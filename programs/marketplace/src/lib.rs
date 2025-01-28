#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use errors::*;
pub use instructions::*;
pub use state::*;

declare_id!("escSmDEY26evSYow7Nio1WtkNFneo95DTq83P4myqer");

#[program]
pub mod marketplace_program {
    use super::*;

    pub fn initialize(mut ctx: Context<InitializeAccounts>, input: InitializeInput) -> Result<()> {
        _initialize(&mut ctx, &input)
    }

    pub fn list(mut ctx: Context<ListAccounts>, input: ListInput) -> Result<()> {
        _list(&mut ctx, input)
    }

    pub fn delist(ctx: Context<DelistAccounts>) -> Result<()> {
        _delist(&ctx)
    }

    pub fn purchase(ctx: Context<PurchaseAccounts>) -> Result<()> {
        _purchase(&ctx)
    }
}
