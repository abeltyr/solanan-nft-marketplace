use {anchor_lang::prelude::*, anchor_spl::token};

use crate::{
    error::ErrorCode,
    processor::english_auction_listing::utils::create_english_auction_listing_pda::*,
    utils::create_nft_listing_pda::*,
};

pub fn create_english_auction_listing_fn(
    ctx: Context<CreateEnglishAuctionListing>,
    start_date: u64,
    end_date: u64,
    starting_price_lamports: u64,
) -> Result<()> {
    msg!("Start the English Auction listing Process");

    // Fetch the nft listing account data and validate the nft status (check if nft is already is listed or not)

    let nft_listing = &ctx.accounts.nft_listing_account.to_account_info();

    let nft_listing_account = &mut ctx.accounts.nft_listing_account;

    let listing_account = &mut ctx.accounts.listing_account;

    let (_pubkey_mint, _) = Pubkey::find_program_address(
        &[listing_account.mint.key().as_ref(), b"_nft_listing_data"],
        ctx.program_id,
    );

    //check if the given nft listing data is the same
    if _pubkey_mint != nft_listing_account.key() {
        return Err(ErrorCode::NftListingInvalidData.into());
    }

    if nft_listing_account.active || listing_account.is_active {
        return Err(ErrorCode::NftAlreadyListed.into());
    }

    // check if the given seller is the same as the one creating the listing pda
    if listing_account.seller != ctx.accounts.seller.key() {
        return Err(ErrorCode::SellerInvalidData.into());
    }

    // start_date cannot be in the past
    if start_date < Clock::get().unwrap().unix_timestamp as u64 {
        return Err(ErrorCode::StartDateIsInPast.into());
    }

    // end_date should not be greater than start_date
    if start_date > end_date {
        return Err(ErrorCode::EndDateIsEarlierThanBeginDate.into());
    }

    //check if listing is closed
    if listing_account.close_date > Some(0) || listing_account.sold.is_some() {
        return Err(ErrorCode::ListingAlreadyClosed.into());
    }

    if ctx.accounts.seller_token.amount != 1 {
        return Err(ErrorCode::MintTokenIssue.into());
    }

    if starting_price_lamports <= 0 {
        return Err(ErrorCode::PriceIssue.into());
    }

    // approve the nft
    token::approve(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Approve {
                authority: ctx.accounts.seller.to_account_info(),
                delegate: nft_listing.to_account_info(),
                to: ctx.accounts.seller_token.to_account_info(),
            },
        ),
        100000000,
    )?;

    // update the listing data
    listing_account.starting_price_lamports = starting_price_lamports;
    listing_account.start_date = Some(start_date);
    listing_account.end_date = Some(end_date);
    listing_account.close_date = Some(0);
    listing_account.highest_bid_lamports = Some(0);
    listing_account.is_active = true;

    // update the nft listing data

    nft_listing_account.amount = nft_listing_account.amount + 1;
    nft_listing_account.active = true;
    nft_listing_account.listing = Some("English Auction".to_string());
    Ok(())
}

#[derive(Accounts)]
pub struct CreateEnglishAuctionListing<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub seller_token: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub nft_listing_account: Account<'info, NftListingData>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    #[account(mut)]
    pub listing_account: Account<'info, EnglishAuctionListingData>,
}
