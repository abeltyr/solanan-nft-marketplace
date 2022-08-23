use {
    anchor_lang::prelude::*,
    anchor_spl::{associated_token, token},
};

use crate::{
    error::ErrorCode, processor::fixed_price_listing::utils::create_fixed_price_listing_pda::*,
    utils::create_nft_listing_pda::*,
};
pub fn close_fixed_price_listing_fn(ctx: Context<CloseFixedPriceListing>) -> Result<()> {
    msg!("Closing The Fixed Price Listing...");

    let listing_account = &mut ctx.accounts.listing_account;
    let nft_listing_account = &mut ctx.accounts.nft_listing_account;

    // check if listing is already closed
    if listing_account.close_date > Some(0)
        || !listing_account.is_active
        || listing_account.sold.is_some()
    {
        return Err(ErrorCode::ListingAlreadyClosed.into());
    }

    let (_pubkey_mint, _) = Pubkey::find_program_address(
        &[listing_account.mint.key().as_ref(), b"_nft_listing_data"],
        ctx.program_id,
    );

    //check if the given nft listing data is the same
    if _pubkey_mint != nft_listing_account.key() {
        return Err(ErrorCode::NftListingInvalidData.into());
    }

    // fetch token account of the owner
    let seller_token = associated_token::get_associated_token_address(
        &listing_account.seller.key(),
        &listing_account.mint.key(),
    );

    // validate the given token address match with the account
    if seller_token.key() != ctx.accounts.seller_token.key() {
        return Err(ErrorCode::InvalidTokenAccount.into());
    }

    // only the seller can close the listing
    // TODO:add an admin so that the admin can also close
    if ctx.accounts.seller.key() != listing_account.seller.key() {
        return Err(ErrorCode::ClosingIssue.into());
    }

    // revoke program nft id
    token::revoke(CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Revoke {
            authority: ctx.accounts.seller.to_account_info(),
            source: ctx.accounts.seller_token.to_account_info(),
        },
    ))?;

    // update the nft listing pda
    nft_listing_account.active = false;
    nft_listing_account.listing = None;

    // close the fixed price listing pda

    let close_date = Clock::get().unwrap().unix_timestamp as u64;
    listing_account.close_date = Some(close_date);
    listing_account.sold = Some(false);
    listing_account.is_active = false;

    Ok(())
}

#[derive(Accounts)]
pub struct CloseFixedPriceListing<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub seller_token: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub nft_listing_account: Account<'info, NftListingData>,
    #[account(mut)]
    pub listing_account: Account<'info, FixedPriceListingData>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
}
