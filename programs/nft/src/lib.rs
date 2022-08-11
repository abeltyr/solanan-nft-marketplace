use anchor_lang::prelude::*;

pub mod processor;

pub mod error;

use crate::processor::{fixed_price_listing::*, nft_listing_pda::*};

declare_id!("HzCCnp6EYNzjeFRBkMucKaqckN13VqZZjXiHSn9N6uws");

#[program]
pub mod listings {
    use super::*;

    pub fn create_nft_listing(ctx: Context<CreateNftListing>) -> Result<()> {
        create_nft_listing_pda(ctx)
    }

    pub fn fixed_price_listing(
        ctx: Context<FixedPriceListing>,
        start_date: u64,
        end_date: u64,
    ) -> Result<()> {
        fixed_price_nft_listing(ctx, start_date, end_date)
    }
}
