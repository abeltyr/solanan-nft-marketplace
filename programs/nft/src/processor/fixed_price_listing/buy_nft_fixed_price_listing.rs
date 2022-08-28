use {
    anchor_lang::{prelude::*, system_program},
    anchor_spl::token,
};

use crate::{
    error::ErrorCode,
    processor::{fixed_price_listing::create_fixed_price_listing::*, nft::mint_nft::*},
    validate::{check_active_listing_data::*, check_listing_is_active::*, check_token_owner::*},
};
pub fn buy_nft_fixed_price_listing_fn(ctx: Context<BuyNftFixedPriceListing>) -> Result<()> {
    msg!("Buy The Nft...");

    // get the account info of the nft listing as an immutable to use for the transfer authority
    let nft_listing = &ctx.accounts.nft_listing_account.to_account_info();

    // get the nft listing as mutable to fetch and update the data
    let nft_listing_account = &mut ctx.accounts.nft_listing_account;

    let listing_account = &mut ctx.accounts.listing_account;

    let nft_listing_pda = check_listing_is_active(
        &ctx.program_id,
        &listing_account.mint,
        listing_account.is_active,
        &nft_listing_account,
    )?;

    let bump_seed = nft_listing_pda.1;

    check_active_listing_data(
        listing_account.start_date,
        listing_account.end_date,
        listing_account.close_date,
        listing_account.price_lamports,
        listing_account.sold,
        &nft_listing,
        &ctx.accounts.seller_token,
    )?;

    // check if the given seller is the same as the one provided in the listing
    if listing_account.seller != ctx.accounts.seller.key() {
        return Err(ErrorCode::SellerInvalidData.into());
    }

    //check seller token match
    check_token_owner(
        &listing_account.seller,
        &ctx.accounts.seller_token,
        &listing_account.mint.key(),
    )?;

    //check buyer token match
    check_token_owner(
        &ctx.accounts.buyer.key(),
        &ctx.accounts.buyer_token,
        &listing_account.mint.key(),
    )?;

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
    listing_account.buyer_token = Some(ctx.accounts.buyer_token.key());

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
