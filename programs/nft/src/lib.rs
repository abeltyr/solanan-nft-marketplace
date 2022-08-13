use anchor_lang::prelude::*;

pub mod processor;

pub mod error;

use crate::processor::{fixed_price_listing::*, fixed_price_listing_pda::*, nft_listing_pda::*};

declare_id!("5q1b5YSiUrWRFuRuPYnqzC2rEUNHtXZpHWwQmSfG24Hs");

#[program]
pub mod listings {
    use super::*;

    pub fn create_nft_listing(ctx: Context<CreateNftListing>) -> Result<()> {
        create_nft_listing_pda(ctx)
    }

    pub fn fixed_price_list(
        ctx: Context<FixedPriceListing>,
        start_date: u64,
        end_date: u64,
        price_lamports: u64,
    ) -> Result<()> {
        fixed_price_list_fn(ctx, start_date, end_date, price_lamports)
    }

    pub fn create_fixed_price_listing_pda(
        ctx: Context<CreateFixedPriceListingPda>,
        count: String,
    ) -> Result<()> {
        create_fixed_price_listing_pda_fn(ctx, count)
    }
}
