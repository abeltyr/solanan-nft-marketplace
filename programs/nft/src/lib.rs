use anchor_lang::prelude::*;

pub mod processor;

pub mod utils;

pub mod error;

use crate::{
    processor::{
        english_auction_listing::{
            close_english_auction_listing::*, create_english_auction_listing::*,
        },
        fixed_price_listing::{
            buy_nft_fixed_price_listing::*, close_fixed_price_listing::*,
            create_fixed_price_listing::*,
        },
    },
    utils::{
        create_english_auction_listing_pda::*, create_fixed_price_listing_pda::*,
        create_nft_listing_pda::*,
    },
};

declare_id!("D3544YeKkf5zB3ENMWiNR62kENQtJhkhCi2CbnruVrBi");

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

    pub fn buy_nft_fixed_price_listing(ctx: Context<BuyNftFixedPriceListing>) -> Result<()> {
        buy_nft_fixed_price_listing_fn(ctx)
    }

    pub fn create_english_auction_listing_pda(
        ctx: Context<CreateEnglishAuctionListingPda>,
        count: String,
    ) -> Result<()> {
        create_english_auction_listing_pda_fn(ctx, count)
    }

    pub fn create_english_auction_listing(
        ctx: Context<CreateEnglishAuctionListing>,
        start_date: u64,
        end_date: u64,
        starting_price_lamports: u64,
    ) -> Result<()> {
        create_english_auction_listing_fn(ctx, start_date, end_date, starting_price_lamports)
    }

    pub fn close_english_auction_listing(ctx: Context<CloseEnglishAuctionListing>) -> Result<()> {
        close_english_auction_listing_fn(ctx)
    }
}
