use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceErrors {
    #[msg("Transfer failed")]
    TransferFailed,
}
