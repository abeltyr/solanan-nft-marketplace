use {
    anchor_lang::{prelude::*, system_program},
    anchor_spl::{associated_token, token},
};

use crate::{
    error::ErrorCode,
    processor::english_auction_listing::utils::{
        create_english_auction_bid_pda::*, create_english_auction_listing_pda::*,
    },
    utils::create_nft_listing_pda::*,
};

pub fn bid_english_auction_fn(
    ctx: Context<BidEnglishAuction>,
    bid_price_lamports: u64,
) -> Result<()> {
    msg!("Bidding process started");

    let nft_listing = &ctx.accounts.nft_listing_account.to_account_info();

    let nft_listing_account = &mut ctx.accounts.nft_listing_account;

    let listing_account = &mut ctx.accounts.listing_account;
    let bid_account = &mut ctx.accounts.bid_account;

    let (_pubkey_mint, _) = Pubkey::find_program_address(
        &[listing_account.mint.key().as_ref(), b"_nft_listing_data"],
        ctx.program_id,
    );

    //check if the given nft listing data is the same
    if _pubkey_mint != nft_listing_account.key() {
        return Err(ErrorCode::NftListingInvalidData.into());
    }

    // check is the nft listing is active
    if !nft_listing_account.active || !listing_account.is_active {
        return Err(ErrorCode::NftNotListed.into());
    }

    // check if the auction is set properly
    if listing_account.start_date.is_none()
        || listing_account.end_date.is_none()
        || listing_account.starting_price_lamports == 0
    {
        return Err(ErrorCode::AuctionNotSet.into());
    }

    // check if the auction is not closed
    if (listing_account.close_date.is_some() && listing_account.close_date > Some(0))
        || listing_account.sold.is_some()
    {
        return Err(ErrorCode::ListingAlreadyClosed.into());
    }

    let current_time = Clock::get().unwrap().unix_timestamp as u64;

    // check if the start date has passed
    if listing_account.start_date.unwrap() > current_time {
        return Err(ErrorCode::AuctionNotStarted.into());
    }

    // check if the end date has not passed
    if listing_account.end_date.unwrap() < current_time {
        return Err(ErrorCode::AuctionEnded.into());
    }

    let mut bid_account_lamports: u64 = bid_price_lamports;

    if bid_account.bid_price_lamports.is_some() {
        bid_account_lamports = bid_account.bid_price_lamports.unwrap() + bid_price_lamports;
    }

    // check if the bid is higher than starting price
    if listing_account.starting_price_lamports > bid_account_lamports {
        return Err(ErrorCode::BidLowerThanStartingBid.into());
    }

    // check if the bid is higher than previous bid
    if listing_account.highest_bid_lamports.is_some()
        && listing_account.highest_bid_lamports.unwrap() >= bid_account_lamports
    {
        return Err(ErrorCode::BidLowerThanHighestBider.into());
    }

    // validate so that the seller can distribute

    let seller_token = associated_token::get_associated_token_address(
        &listing_account.seller.clone(),
        &listing_account.mint.clone(),
    );

    if seller_token.key() != ctx.accounts.seller_token.key() {
        return Err(ErrorCode::InvalidTokenAccount.into());
    }

    if bid_account.bidder.key() != ctx.accounts.bidder.key() {
        return Err(ErrorCode::BidderInvalidData.into());
    }

    // validate if the token is still under the owner by the token account
    if ctx.accounts.seller_token.delegate.is_none()
        || ctx.accounts.seller_token.delegate.unwrap() != nft_listing.key()
        || ctx.accounts.seller_token.delegated_amount != 100000000
        || ctx.accounts.seller_token.amount != 1
    {
        return Err(ErrorCode::InvalidTokenAccountDelegation.into());
    }

    // transfer the fund
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.bidder.to_account_info(),
                to: ctx.accounts.bid_account_vault.to_account_info(),
            },
        ),
        bid_price_lamports,
    )?;

    // fetch token account of the seller
    let bidder_token = associated_token::get_associated_token_address(
        &ctx.accounts.bidder.key(),
        &listing_account.mint.clone(),
    );

    if bidder_token.key() != ctx.accounts.bidder_token.key() {
        return Err(ErrorCode::InvalidTokenAccount.into());
    }

    bid_account.bidder_token = bidder_token.key();
    bid_account.bid_price_lamports = Some(bid_account_lamports);
    bid_account.fund_deposit = Some(true);

    listing_account.highest_bid_pda = Some(ctx.accounts.bid_account.key().clone());
    listing_account.highest_bid_lamports = Some(bid_account_lamports);
    listing_account.highest_bidder = Some(ctx.accounts.bidder.key());
    listing_account.highest_bidder_token = Some(bidder_token.key());

    Ok(())
}

#[derive(Accounts)]
pub struct BidEnglishAuction<'info> {
    #[account(mut)]
    pub nft_listing_account: Account<'info, NftListingData>,
    #[account(mut)]
    pub listing_account: Account<'info, EnglishAuctionListingData>,
    #[account(mut)]
    pub bidder: Signer<'info>,
    #[account(mut)]
    pub bidder_token: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub seller_token: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub bid_account: Account<'info, EnglishAuctionListingBidData>,
    #[account(mut)]
    /// CHECK:
    pub bid_account_vault: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
}
