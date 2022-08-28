use {anchor_lang::prelude::*, anchor_spl::token};

use crate::{
    processor::{fixed_price_listing::create_fixed_price_listing::*, nft::mint_nft::*},
    validate::{check_listing_closing::*, check_token_owner::*},
};
pub fn close_fixed_price_listing_fn(ctx: Context<CloseFixedPriceListing>) -> Result<()> {
    msg!("Closing The Fixed Price Listing...");

    let listing_account = &mut ctx.accounts.listing_account;
    let nft_listing_account = &mut ctx.accounts.nft_listing_account;

    check_token_owner(
        &listing_account.seller.clone(),
        &ctx.accounts.seller_token,
        &nft_listing_account.mint.key(),
    )?;

    check_listing_closing(
        &ctx.accounts.closer,
        &listing_account.seller.clone(),
        listing_account.close_date,
        listing_account.is_active,
        listing_account.sold,
    )?;

    if ctx.accounts.closer.key() == listing_account.seller {
        // revoke program token id
        token::revoke(CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Revoke {
                authority: ctx.accounts.closer.to_account_info(),
                source: ctx.accounts.seller_token.to_account_info(),
            },
        ))?;
    }

    // update the nft listing pda
    nft_listing_account.active = false;
    nft_listing_account.listing = None;

    // close the fixed price listing pda
    listing_account.close_date = Some(Clock::get().unwrap().unix_timestamp as u64);
    listing_account.sold = Some(false);
    listing_account.is_active = false;

    Ok(())
}

#[derive(Accounts)]
pub struct CloseFixedPriceListing<'info> {
    #[account(mut)]
    pub closer: Signer<'info>,
    #[account(mut)]
    pub seller_token: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub nft_listing_account: Account<'info, NftListingData>,
    #[account(mut)]
    pub listing_account: Account<'info, FixedPriceListingData>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
}
