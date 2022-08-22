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
    msg!("Start the English Auction listing Process");

    let nft_listing = &ctx.accounts.nft_listing_account.to_account_info();

    let auction_account = &mut ctx.accounts.auction_account;
    let bid_account = &mut ctx.accounts.bid_account;

    if !auction_account.is_active {
        return Err(ErrorCode::NftNotListed.into());
    }

    // check if the auction is set properly
    if auction_account.start_date.is_none()
        || auction_account.end_date.is_none()
        || auction_account.starting_price_lamports == 0
    {
        return Err(ErrorCode::AuctionNotSet.into());
    }

    // check if the auction is not closed
    if (auction_account.close_date.is_some() && auction_account.close_date > Some(0))
        || auction_account.sold.is_some()
        || auction_account.fund_withdrawn.is_some()
    {
        return Err(ErrorCode::ListingAlreadyClosed.into());
    }

    let clock = Clock::get().unwrap().unix_timestamp as u64;

    // check if the start date has passed
    if auction_account.start_date.unwrap() > clock {
        return Err(ErrorCode::AuctionNotStarted.into());
    }

    // check if the end date has not passed
    if auction_account.end_date.unwrap() < clock {
        return Err(ErrorCode::AuctionEnded.into());
    }

    let mut bid_account_lamports: u64 = bid_price_lamports;

    if bid_account.bid_price_lamports.is_some() {
        bid_account_lamports = bid_account.bid_price_lamports.unwrap() + bid_price_lamports;
    }

    // check if the bid is higher than starting price
    if auction_account.starting_price_lamports > bid_account_lamports {
        return Err(ErrorCode::BidLowerThanStartingBid.into());
    }

    // check if the bid is higher than previous bid
    if auction_account.highest_bid_lamports.is_some()
        && auction_account.highest_bid_lamports.unwrap() >= bid_account_lamports
    {
        return Err(ErrorCode::BidLowerThanHighestBider.into());
    }

    // validate so that the seller can distribute

    let seller_token_account = associated_token::get_associated_token_address(
        &auction_account.seller.clone(),
        &auction_account.mint.clone(),
    );

    if seller_token_account.key() != ctx.accounts.seller_token_account.key() {
        return Err(ErrorCode::InvalidTokenAccount.into());
    }

    // validate if the token is still under the owner by the token account

    if ctx.accounts.seller_token_account.delegate.is_none()
        || ctx.accounts.seller_token_account.delegate.unwrap() != nft_listing.key()
        || ctx.accounts.seller_token_account.delegated_amount != 100000000
        || ctx.accounts.seller_token_account.amount != 1
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
    let bidder_token_account = associated_token::get_associated_token_address(
        &ctx.accounts.bidder.key(),
        &auction_account.mint.clone(),
    );

    if bidder_token_account.key() != ctx.accounts.bidder_token_account.key() {
        return Err(ErrorCode::InvalidTokenAccount.into());
    }

    bid_account.bidder_token = bidder_token_account.key();
    bid_account.bid_price_lamports = Some(bid_account_lamports);
    bid_account.bid_date = Some(clock);
    bid_account.fund_deposit = Some(true);

    auction_account.highest_bid_pda = Some(ctx.accounts.bid_account.key().clone());
    auction_account.highest_bid_lamports = Some(bid_account_lamports);
    auction_account.highest_bidder = Some(ctx.accounts.bidder.key());
    auction_account.highest_bidder_token = Some(bidder_token_account.key());

    Ok(())
}

#[derive(Accounts)]
pub struct BidEnglishAuction<'info> {
    #[account(mut)]
    pub bidder: Signer<'info>,
    #[account(mut)]
    pub nft_listing_account: Account<'info, NftListingData>,
    #[account(mut)]
    pub bidder_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub seller_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub auction_account: Account<'info, EnglishAuctionListingData>,
    #[account(mut)]
    pub bid_account: Account<'info, EnglishAuctionListingBidData>,
    #[account(mut)]
    /// CHECK:
    pub bid_account_vault: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
}
