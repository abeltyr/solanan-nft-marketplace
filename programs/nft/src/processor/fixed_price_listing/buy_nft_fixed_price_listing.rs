use {
    anchor_lang::{prelude::*, system_program},
    anchor_spl::{associated_token, token},
};

use crate::{
    error::ErrorCode, processor::fixed_price_listing::utils::create_fixed_price_listing_pda::*,
    utils::create_nft_listing_pda::*,
};
pub fn buy_nft_fixed_price_listing_fn(ctx: Context<BuyNftFixedPriceListing>) -> Result<()> {
    msg!("Buy The Nft...");

    // get the account info of the nft listing as an immutable to use for the transfer authority
    let nft_listing = &ctx.accounts.nft_listing_account.to_account_info();

    // get the nft listing as mutable to fetch and update the data
    let nft_listing_account = &mut ctx.accounts.nft_listing_account;

    let listing_account = &mut ctx.accounts.listing_account;

    // check is the nft listing is active
    if !nft_listing_account.active {
        return Err(ErrorCode::NftNotListed.into());
    }

    // check is the listing has the price setup
    if listing_account.price_lamports == 0 {
        return Err(ErrorCode::ListingPriceNotSet.into());
    }

    // check is the listing is not closed
    if listing_account.close_date > Some(0)
        || listing_account.sold.is_some()
        || listing_account.fund_sent.is_some()
    {
        return Err(ErrorCode::ListingAlreadyClosed.into());
    }

    // check is the listing is not closed
    if listing_account.start_date == Some(0)
        || listing_account.end_date == Some(0)
        || !listing_account.is_active
    {
        return Err(ErrorCode::ListingNotActivate.into());
    }

    //get the bump seed for the signed transaction
    let (_pubkey_mint, bump_seed) = Pubkey::find_program_address(
        &[listing_account.mint.key().as_ref(), b"_nft_listing_data"],
        ctx.program_id,
    );

    //check if the given nft listing data is the same
    if _pubkey_mint != nft_listing.key() {
        return Err(ErrorCode::NftListingInvalidData.into());
    }

    // check if the given seller is the same as the one provided in the listing
    if listing_account.seller != ctx.accounts.seller.key() {
        return Err(ErrorCode::SellerInvalidData.into());
    }

    let seller_token = associated_token::get_associated_token_address(
        &listing_account.seller.key(),
        &listing_account.mint.key(),
    );

    let buyer_token = associated_token::get_associated_token_address(
        &ctx.accounts.buyer.key(),
        &listing_account.mint.key(),
    );

    // check the given token address match and has the proper authority
    if seller_token.key() != ctx.accounts.seller_token.key()
        || buyer_token.key() != ctx.accounts.buyer_token.key()
    {
        return Err(ErrorCode::InvalidTokenAccount.into());
    }

    // check the given token address has access to the nft and that it has given delegation authority
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
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.seller.to_account_info(),
            },
        ),
        listing_account.price_lamports,
    )?;

    // update the listing data according to the fund transfer
    listing_account.fund_sent = Some(true);
    listing_account.buyer = Some(ctx.accounts.buyer.key());

    // transfer the NFT To buyer
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.seller_token.to_account_info(),
                to: ctx.accounts.buyer_token.to_account_info(),
                authority: nft_listing.to_account_info(),
            },
            &[&[
                listing_account.mint.key().as_ref(),
                b"_nft_listing_data",
                &[bump_seed],
            ]],
        ),
        1,
    )?;

    // close the nft listing
    nft_listing_account.active = false;
    nft_listing_account.listing = None;

    // close the listing
    listing_account.close_date = Some(Clock::get().unwrap().unix_timestamp as u64);
    listing_account.sold = Some(true);
    listing_account.is_active = false;
    listing_account.buyer_token = Some(buyer_token.key());

    Ok(())
}

#[derive(Accounts)]
pub struct BuyNftFixedPriceListing<'info> {
    #[account(mut)]
    pub nft_listing_account: Account<'info, NftListingData>,
    #[account(mut)]
    pub listing_account: Account<'info, FixedPriceListingData>,
    #[account(mut)]
    /// CHECK:
    pub seller: UncheckedAccount<'info>,
    #[account(mut)]
    pub seller_token: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub buyer_token: Account<'info, token::TokenAccount>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
}
