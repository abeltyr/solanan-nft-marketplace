use {anchor_lang::prelude::*, anchor_spl::token};

use crate::{
    error::ErrorCode, processor::fixed_price_listing::utils::create_fixed_price_listing_pda::*,
    utils::create_nft_listing_pda::*,
};

pub fn create_fixed_price_listing_fn(
    ctx: Context<CreateFixedPriceListing>,
    start_date: u64,
    end_date: u64,
    price_lamports: u64,
) -> Result<()> {
    msg!("Start the Fixed Price listing Process");

    // Fetch the nft listing account data and validate the nft status (check if nft is already is listed or not)

    let nft_listing = &ctx.accounts.nft_listing_account.to_account_info();

    let nft_listing_account = &mut ctx.accounts.nft_listing_account;

    msg!("nft_listing_account amount: {}", nft_listing_account.amount);
    msg!("nft_listing_account active: {}", nft_listing_account.active);

    if nft_listing_account.active {
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

    if ctx.accounts.owner_token_account.amount != 1 {
        return Err(ErrorCode::MintTokenIssue.into());
    }

    // approve the nft
    token::approve(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Approve {
                authority: ctx.accounts.owner.to_account_info(),
                delegate: nft_listing.to_account_info(),
                to: ctx.accounts.owner_token_account.to_account_info(),
            },
        ),
        100000000,
    )?;

    // update the listing data
    let listing_account = &mut ctx.accounts.listing_account;
    listing_account.price_lamports = price_lamports;
    listing_account.start_date = Some(start_date);
    listing_account.end_date = Some(end_date);
    listing_account.is_active = true;

    // // update the nft listing data

    nft_listing_account.amount = nft_listing_account.amount + 1;
    nft_listing_account.active = true;
    nft_listing_account.listing = Some("Fixed Price".to_string());
    Ok(())
}

#[derive(Accounts)]
pub struct CreateFixedPriceListing<'info> {
    #[account(mut)]
    pub owner_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub nft_listing_account: Account<'info, NftListingData>,
    #[account(mut)]
    pub listing_account: Account<'info, FixedPriceListingData>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
}
