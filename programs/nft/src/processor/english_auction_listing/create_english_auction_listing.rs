use {anchor_lang::prelude::*, anchor_spl::token};

use crate::{
    error::ErrorCode,
    utils::{create_english_auction_listing_pda::*, create_nft_listing_pda::*},
};

pub fn create_english_auction_listing_fn(
    ctx: Context<CreateEnglishAuctionListing>,
    start_date: u64,
    end_date: u64,
    starting_price_lamports: u64,
) -> Result<()> {
    msg!("Start the Fixed Price listing Process");

    // Fetch the nft listing account data and validate the nft status (check if nft is already is listed or not)

    let nft_listing_account = &mut ctx.accounts.nft_listing_account;
    let listing_account = &mut ctx.accounts.listing_account;

    if nft_listing_account.active || listing_account.is_active {
        return Err(ErrorCode::NftAlreadyListed.into());
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
    if listing_account.close_date > Some(0)
        || listing_account.sold.is_some()
        || listing_account.fund_withdrawn.is_some()
    {
        return Err(ErrorCode::ListingAlreadyClosed.into());
    }
    // approve the nft
    token::approve(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Approve {
                authority: ctx.accounts.owner.to_account_info(),
                delegate: ctx.accounts.program_account.to_account_info(),
                to: ctx.accounts.owner_token_account.to_account_info(),
            },
        ),
        100000000,
    )?;

    // update the listing data
    listing_account.starting_price_lamports = starting_price_lamports;
    listing_account.start_date = Some(start_date);
    listing_account.end_date = Some(end_date);
    listing_account.is_active = true;

    // update the nft listing data

    nft_listing_account.amount = nft_listing_account.amount + 1;
    nft_listing_account.active = true;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateEnglishAuctionListing<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub owner_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub nft_listing_account: Account<'info, NftListingData>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    #[account()]
    pub program_account: Signer<'info>,
    #[account(mut)]
    pub listing_account: Account<'info, EnglishAuctionListingData>,
}
