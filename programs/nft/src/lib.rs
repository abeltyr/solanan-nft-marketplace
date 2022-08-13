use anchor_lang::prelude::*;

pub mod processor;

pub mod error;

use crate::processor::{
    close_fixed_price_listing::*, create_fixed_price_listing::*, create_fixed_price_listing_pda::*,
    create_nft_listing_pda::*,
};

declare_id!("8BMStoEUDDeLG6DhcL1z51Vgoab2mWcCTHmupr7URDhv");

#[program]
pub mod listings {
    use super::*;

    pub fn create_nft_listing_pda(ctx: Context<CreateNftListingPda>) -> Result<()> {
        create_nft_listing_pda_fn(ctx)
    }

    pub fn create_fixed_price_listing(
        ctx: Context<CreateFixedPriceListing>,
        start_date: u64,
        end_date: u64,
        price_lamports: u64,
    ) -> Result<()> {
        create_fixed_price_listing_fn(ctx, start_date, end_date, price_lamports)
    }

    pub fn create_fixed_price_listing_pda(
        ctx: Context<CreateFixedPriceListingPda>,
        count: String,
    ) -> Result<()> {
        create_fixed_price_listing_pda_fn(ctx, count)
    }

    pub fn close_fixed_price_listing(ctx: Context<CloseFixedPriceListing>) -> Result<()> {
        close_fixed_price_listing_fn(ctx)
    }
}
