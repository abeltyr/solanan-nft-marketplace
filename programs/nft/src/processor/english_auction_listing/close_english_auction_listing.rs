use {
    anchor_lang::prelude::*,
    anchor_spl::{associated_token, token},
};

use crate::{
    error::ErrorCode,
    utils::{create_english_auction_listing_pda::*, create_nft_listing_pda::*},
};
pub fn close_english_auction_listing_fn(ctx: Context<CloseEnglishAuctionListing>) -> Result<()> {
    msg!("Closing The English Auction Listing...");

    // fetch token account of the owner
    let owner_token_account = associated_token::get_associated_token_address(
        &ctx.accounts.owner.key(),
        &ctx.accounts.mint.key(),
    );

    if owner_token_account.key() != ctx.accounts.owner_token_account.key() {
        return Err(ErrorCode::InvalidTokenAccount.into());
    }

    if ctx.accounts.owner_token_account.amount != 1 {
        return Err(ErrorCode::TokenAccountOwnerIssue.into());
    }

    let nft_listing_account = &mut ctx.accounts.nft_listing_account;
    let listing_account = &mut ctx.accounts.listing_account;

    //check if listing is already closed
    if listing_account.close_date > Some(0)
        || listing_account.sold.is_some()
        || listing_account.fund_withdrawn.is_some()
    {
        return Err(ErrorCode::ListingAlreadyClosed.into());
    }

    // revoke program nft id
    token::revoke(CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Revoke {
            authority: ctx.accounts.owner.to_account_info(),
            source: ctx.accounts.owner_token_account.to_account_info(),
        },
    ))?;

    // update the nft listing pda
    nft_listing_account.active = false;

    // close the fixed price listing pda
    listing_account.close_date = Some(Clock::get().unwrap().unix_timestamp as u64);
    listing_account.sold = Some(false);
    listing_account.is_active = false;

    Ok(())
}

#[derive(Accounts)]
pub struct CloseEnglishAuctionListing<'info> {
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub owner_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub nft_listing_account: Account<'info, NftListingData>,
    #[account(mut)]
    pub listing_account: Account<'info, EnglishAuctionListingData>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
}
