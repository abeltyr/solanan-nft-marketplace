use {
    anchor_lang::{prelude::*, system_program},
    anchor_spl::{associated_token, token},
};

use crate::{
    error::ErrorCode,
    processor::fixed_price_listing::utils::{
        create_fixed_price_listing_pda::*, create_nft_listing_pda::*,
    },
};
pub fn buy_nft_fixed_price_listing_fn(ctx: Context<BuyNftFixedPriceListing>) -> Result<()> {
    msg!("Buy The Nft...");

    // validate the nft and listing data
    let nft_listing_account = &mut ctx.accounts.nft_listing_account;
    let listing_account = &mut ctx.accounts.listing_account;
    msg!(
        "checking if nft_listing_account is not active: {}",
        !nft_listing_account.active
    );
    if !nft_listing_account.active {
        return Err(ErrorCode::NftNotListed.into());
    }

    msg!(
        "checking listing_account price: {}",
        listing_account.price_lamports == 0
    );
    if listing_account.price_lamports == 0 {
        return Err(ErrorCode::ListingPriceNotSet.into());
    }

    msg!(
        "checking listing is closed: {}",
        listing_account.close_date > Some(0)
            || listing_account.sold.is_some()
            || listing_account.fund_sent.is_some()
    );

    if listing_account.close_date > Some(0)
        || listing_account.sold.is_some()
        || listing_account.fund_sent.is_some()
    {
        return Err(ErrorCode::ListingAlreadyClosed.into());
    }

    msg!(
        "checking listing_account is not activated: {}",
        listing_account.start_date == Some(0)
            || listing_account.end_date == Some(0)
            || !listing_account.is_active
    );

    if listing_account.start_date == Some(0)
        || listing_account.end_date == Some(0)
        || !listing_account.is_active
    {
        return Err(ErrorCode::ListingNotActivate.into());
    }

    //--------------------------------------------------------//

    // check the given token address match and has the proper authority
    let seller_token_account = associated_token::get_associated_token_address(
        &listing_account.seller.key(),
        &ctx.accounts.mint.key(),
    );

    let buyer_token_account = associated_token::get_associated_token_address(
        &ctx.accounts.buyer.key(),
        &ctx.accounts.mint.key(),
    );

    if seller_token_account.key() != ctx.accounts.seller_token_account.key()
        || buyer_token_account.key() != ctx.accounts.buyer_token_account.key()
    {
        return Err(ErrorCode::InvalidTokenAccount.into());
    }

    if ctx.accounts.seller_token_account.delegate.is_none()
        || ctx.accounts.seller_token_account.delegate.unwrap() != ctx.accounts.program_account.key()
        || ctx.accounts.seller_token_account.delegated_amount != 100000000
        || ctx.accounts.seller_token_account.amount != 1
    {
        return Err(ErrorCode::InvalidTokenAccountDelegation.into());
    }

    //--------------------------------------------------------//

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

    listing_account.fund_sent = Some(true);
    listing_account.buyer = Some(ctx.accounts.buyer.key());

    //--------------------------------------------------------//
    // transfer the NFT To buyer
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.seller_token_account.to_account_info(),
                to: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.program_account.to_account_info(),
            },
        ),
        1,
    )?;

    //--------------------------------------------------------//
    // update the nft and listing pda
    nft_listing_account.active = false;

    listing_account.close_date = Some(Clock::get().unwrap().unix_timestamp as u64);
    listing_account.sold = Some(true);
    listing_account.is_active = false;
    listing_account.buyer_token = Some(buyer_token_account.key());

    Ok(())
}

#[derive(Accounts)]
pub struct BuyNftFixedPriceListing<'info> {
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub nft_listing_account: Account<'info, NftListingData>,
    #[account(mut)]
    pub listing_account: Account<'info, FixedPriceListingData>,
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub seller_token_account: Account<'info, token::TokenAccount>,
    #[account()]
    pub program_account: Signer<'info>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, token::TokenAccount>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
}
