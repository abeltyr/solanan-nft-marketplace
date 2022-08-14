use {
    anchor_lang::prelude::*,
    anchor_spl::{associated_token, token},
};

use crate::{
    error::ErrorCode,
    processor::{create_fixed_price_listing_pda::*, create_nft_listing_pda::*},
};

pub fn close_fixed_price_listing_fn(ctx: Context<CloseFixedPriceListing>) -> Result<()> {
    msg!("Closing The Fixed Price Listing...");

    // fetch token account of the owner
    let owner_token_account = associated_token::get_associated_token_address(
        &ctx.accounts.owner.key(),
        &ctx.accounts.mint.key(),
    );

    if owner_token_account.key() != ctx.accounts.owner_token_account.key() {
        return Err(ErrorCode::InvalidTokenAccount.into());
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
    let nft_listing_account = &mut ctx.accounts.nft_listing_account;
    nft_listing_account.active = false;

    // close the fixed price listing pda

    let close_date = Clock::get().unwrap().unix_timestamp as u64;
    let listing_account = &mut ctx.accounts.listing_account;
    listing_account.close_date = Some(close_date);
    listing_account.sold = Some(false);
    listing_account.is_active = false;

    Ok(())
}

#[derive(Accounts)]
pub struct CloseFixedPriceListing<'info> {
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub owner_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub nft_listing_account: Account<'info, NftListingData>,
    #[account(mut)]
    pub listing_account: Account<'info, FixedPriceListingData>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
}
