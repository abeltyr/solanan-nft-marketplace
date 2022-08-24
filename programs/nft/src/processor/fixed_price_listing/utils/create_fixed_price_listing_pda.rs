use {anchor_lang::prelude::*, anchor_spl::token};

use crate::{
    error::ErrorCode::NftAlreadyListed, utils::create_nft_listing_pda::*,
    validate::check_nft_owner::*,
};

pub fn create_fixed_price_listing_pda_fn(ctx: Context<CreateFixedPriceListingPda>) -> Result<()> {
    msg!("Start Fixed Price nft Listing PDA",);

    let nft_listing_account = &mut ctx.accounts.nft_listing_account;

    // check if the nft is not already listed before creating the PDA
    if nft_listing_account.active {
        return Err(NftAlreadyListed.into());
    }

    // fetch token account of the seller and check owner
    check_nft_owner(
        &ctx.accounts.seller,
        &ctx.accounts.seller_token,
        nft_listing_account,
    )?;

    // update the listing data
    let listing_account = &mut ctx.accounts.listing_account;
    listing_account.seller = ctx.accounts.seller.key();
    listing_account.mint = nft_listing_account.mint;
    listing_account.seller_token = ctx.accounts.seller_token.key();
    listing_account.price_lamports = 0;
    listing_account.start_date = Some(0);
    listing_account.end_date = Some(0);
    listing_account.close_date = Some(0);
    listing_account.is_active = false;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateFixedPriceListingPda<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub seller_token: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub nft_listing_account: Account<'info, NftListingData>,
    #[account(
        init,
        payer = seller,
        space = 250,
        seeds = [
            nft_listing_account.key().as_ref(),
            b"_Fixed_Price_",
            nft_listing_account.amount.to_string().as_ref(),
        ],
        bump
    )]
    pub listing_account: Account<'info, FixedPriceListingData>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct FixedPriceListingData {
    pub mint: Pubkey,
    pub seller: Pubkey,
    pub seller_token: Pubkey,
    pub buyer: Option<Pubkey>,
    pub buyer_token: Option<Pubkey>,
    pub price_lamports: u64,
    pub start_date: Option<u64>,
    pub end_date: Option<u64>,
    pub close_date: Option<u64>,
    pub sold: Option<bool>,
    pub is_active: bool,
    pub fund_sent: Option<bool>,
}
