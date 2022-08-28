use anchor_lang::prelude::*;

pub mod processor;

pub mod error;

pub mod validate;

use crate::processor::{
    english_auction_listing::{
        bid_english_auction::*,
        close_english_auction_listing::*,
        create_english_auction_listing::*,
        utils::{create_english_auction_bid_pda::*, create_english_auction_listing_pda::*},
        withdraw_bid_english_auction::*,
    },
    fixed_price_listing::{
        buy_nft_fixed_price_listing::*, close_fixed_price_listing::*, create_fixed_price_listing::*,
    },
    nft::{mint_nft::*, setup_nft_metadata::*},
};

declare_id!("BunPDquq7AxQsF3uxfGmNp6HQQ1rvAHw34RUQx5wa4C3");

#[program]
pub mod listings {
    use super::*;

    pub fn create_fixed_price_listing(
        ctx: Context<CreateFixedPriceListing>,
        start_date: u64,
        end_date: u64,
        price_lamports: u64,
    ) -> Result<()> {
        create_fixed_price_listing_fn(ctx, start_date, end_date, price_lamports)
    }

    pub fn close_fixed_price_listing(ctx: Context<CloseFixedPriceListing>) -> Result<()> {
        close_fixed_price_listing_fn(ctx)
    }

    pub fn buy_nft_fixed_price_listing(ctx: Context<BuyNftFixedPriceListing>) -> Result<()> {
        buy_nft_fixed_price_listing_fn(ctx)
    }

    pub fn create_english_auction_listing_pda(
        ctx: Context<CreateEnglishAuctionListingPda>,
    ) -> Result<()> {
        create_english_auction_listing_pda_fn(ctx)
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

    pub fn create_english_auction_bid_pda(ctx: Context<CreateEnglishAuctionBidPda>) -> Result<()> {
        create_english_auction_bid_pda_fn(ctx)
    }
    pub fn bid_english_auction(
        ctx: Context<BidEnglishAuction>,
        bid_price_lamports: u64,
    ) -> Result<()> {
        bid_english_auction_fn(ctx, bid_price_lamports)
    }

    pub fn withdraw_bid_english_auction(ctx: Context<WithdrawBidEnglishAuction>) -> Result<()> {
        withdraw_bid_english_auction_fn(ctx)
    }

    pub fn mint_nft(ctx: Context<MintNft>) -> Result<()> {
        mint_nft_fn(ctx)
    }

    pub fn setup_nft_metadata(
        ctx: Context<SetupNftMetadata>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
    ) -> Result<()> {
        setup_nft_metadata_fn(ctx, metadata_title, metadata_symbol, metadata_uri)
    }
}
