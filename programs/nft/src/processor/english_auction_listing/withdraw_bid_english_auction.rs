use anchor_lang::prelude::*;

use crate::{
    error::ErrorCode,
    processor::english_auction_listing::utils::{
        create_english_auction_bid_pda::*, create_english_auction_listing_pda::*,
    },
};

pub fn withdraw_bid_english_auction_fn(ctx: Context<WithdrawBidEnglishAuction>) -> Result<()> {
    msg!("Withdraw the bid Process");

    let auction_account = &mut ctx.accounts.auction_account;
    let bid_account = &mut ctx.accounts.bid_account;

    let current_time = Clock::get().unwrap().unix_timestamp as u64;

    // check if the auction is active
    if auction_account.is_active {
        return Err(ErrorCode::ActiveListing.into());
    }

    // check if the auction is closed by check if the closed date has passed
    if auction_account.close_date.is_none()
        || (auction_account.close_date.is_some()
            && (auction_account.close_date.unwrap() == 0
                || auction_account.close_date.unwrap() > current_time))
    {
        return Err(ErrorCode::ListingNotClosed.into());
    }

    //validate the highest bidder withdrawal

    // should all the lamports be extract and close the bid account at the same time

    let bid_account_lamports = **ctx.accounts.bid_account_vault.lamports.borrow();

    // check if the bid has lamports deposited
    if bid_account.bid_price_lamports.is_none()
        || bid_account.bid_price_lamports.is_some() && bid_account.bid_price_lamports.unwrap() == 0
    {
        return Err(ErrorCode::NOLamports.into());
    }

    if auction_account.highest_bid_pda.is_none() {
        return Err(ErrorCode::ListingNotClosed.into());
    }

    if auction_account.seller != ctx.accounts.withdrawer.key()
        && bid_account.bidder != ctx.accounts.withdrawer.key()
    {
        return Err(ErrorCode::UnAuthorizedWithdrawal.into());
    }

    if auction_account.seller != ctx.accounts.withdrawer.key()
        && auction_account.highest_bid_pda.unwrap() == ctx.accounts.bid_account_vault.key()
    {
        return Err(ErrorCode::HighestBidderWithDrawIssue.into());
    }
    if auction_account.seller == ctx.accounts.withdrawer.key()
        && auction_account.highest_bid_pda.unwrap() != ctx.accounts.bid_account_vault.key()
    {
        return Err(ErrorCode::BidAccountIssue.into());
    }

    msg!(
        "bid_price_lamports :{} bid_account_lamports :{}",
        bid_account.bid_price_lamports.unwrap(),
        bid_account_lamports
    );
    **ctx.accounts.bid_account_vault.try_borrow_mut_lamports()? -=
        bid_account.bid_price_lamports.unwrap();
    **ctx.accounts.withdrawer.try_borrow_mut_lamports()? += bid_account.bid_price_lamports.unwrap();

    // bid_account.withdrawn_by = some(ctx.accounts.withdrawer.key().clone());

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawBidEnglishAuction<'info> {
    #[account(mut)]
    pub withdrawer: Signer<'info>,
    #[account(mut)]
    pub auction_account: Account<'info, EnglishAuctionListingData>,
    #[account(mut)]
    pub bid_account: Account<'info, EnglishAuctionListingBidData>,
    #[account(mut)]
    /// CHECK:
    pub bid_account_vault: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
