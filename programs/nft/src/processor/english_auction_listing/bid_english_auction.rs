use {
    anchor_lang::{prelude::*, system_program},
    anchor_spl::{associated_token, token},
};

use crate::{
    error::ErrorCode,
    utils::{create_english_auction_bid_pda::*, create_english_auction_listing_pda::*},
};

pub fn bid_english_auction_fn(
    ctx: Context<BidEnglishAuction>,
    bid_price_lamports: u64,
) -> Result<()> {
    msg!("Start the English Auction listing Process");

    // Fetch the nft listing account data and validate the nft status (check if nft is already is listed or not)

    let english_listing_account = &mut ctx.accounts.english_listing_account;
    let english_listing_bid_account = &mut ctx.accounts.english_listing_bid_account;

    if !english_listing_account.is_active {
        return Err(ErrorCode::NftNotListed.into());
    }

    let clock = Clock::get().unwrap().unix_timestamp as u64;

    if english_listing_account.start_date.is_none()
        || english_listing_account.end_date.is_none()
        || english_listing_account.close_date.is_none()
        || english_listing_account.highest_bid_lamports.is_none()
    {
        return Err(ErrorCode::InvalidData.into());
    }

    if english_listing_account.start_date.unwrap() > clock {
        return Err(ErrorCode::AuctionNotStarted.into());
    }

    if english_listing_account.end_date.unwrap() <= clock {
        return Err(ErrorCode::AuctionEnded.into());
    }

    //check if listing is closed
    if english_listing_account.close_date > Some(0)
        || english_listing_account.sold.is_some()
        || english_listing_account.fund_withdrawn.is_some()
    {
        return Err(ErrorCode::ListingAlreadyClosed.into());
    }

    if english_listing_account.highest_bid_lamports.unwrap() <= bid_price_lamports {
        return Err(ErrorCode::BidLowerThanHighestBider.into());
    }

    // transfer the fund
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.bidder.to_account_info(),
                to: ctx
                    .accounts
                    .english_listing_bid_account_vault
                    .to_account_info(),
            },
        ),
        bid_price_lamports,
    )?;

    // fetch token account of the seller
    let bidder_token_account = associated_token::get_associated_token_address(
        &ctx.accounts.bidder.key(),
        &ctx.accounts.mint.key(),
    );

    if bidder_token_account.key() != ctx.accounts.bidder_token_account.key() {
        return Err(ErrorCode::InvalidTokenAccount.into());
    }

    english_listing_bid_account.bidder_token = bidder_token_account.key();
    english_listing_bid_account.bid_price_lamports = Some(bid_price_lamports);
    english_listing_bid_account.bid_date = Some(clock);
    english_listing_bid_account.fund_deposit = Some(true);

    english_listing_account.highest_bid_lamports = Some(bid_price_lamports);
    english_listing_account.highest_bidder = Some(ctx.accounts.bidder.key());
    english_listing_account.highest_bidder_token = Some(bidder_token_account.key());

    Ok(())
}

#[derive(Accounts)]
pub struct BidEnglishAuction<'info> {
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub bidder: Signer<'info>,
    #[account(mut)]
    pub bidder_token_account: Account<'info, token::TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    #[account(mut)]
    pub english_listing_account: Account<'info, EnglishAuctionListingData>,
    #[account(mut)]
    pub english_listing_bid_account: Account<'info, EnglishAuctionListingBidData>,
    #[account(mut)]
    /// CHECK:
    pub english_listing_bid_account_vault: UncheckedAccount<'info>,
}
