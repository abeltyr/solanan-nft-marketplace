use {
    anchor_lang::prelude::*,
    anchor_spl::{associated_token, token},
};

use crate::{
    error::ErrorCode::{MintTokenIssue, NftAlreadyListed},
    utils::create_nft_listing_pda::*,
};

pub fn create_fixed_price_listing_pda_fn(
    ctx: Context<CreateFixedPriceListingPda>,
    count: String,
) -> Result<()> {
    msg!("Fixed Price nft Listing count:{}...", count);

    let nft_listing_account = &mut ctx.accounts.nft_listing_account;

    // check if the nft is not already listed before creating the PDA
    if nft_listing_account.active {
        return Err(NftAlreadyListed.into());
    }

    // fetch token account of the seller
    let seller_token = associated_token::get_associated_token_address(
        &ctx.accounts.seller.key(),
        &nft_listing_account.mint.key(),
    );

    if ctx.accounts.seller_token.amount != 1 {
        return Err(MintTokenIssue.into());
    }

    // update the listing data
    let listing_account = &mut ctx.accounts.listing_account;
    listing_account.seller = ctx.accounts.seller.key();
    listing_account.mint = nft_listing_account.mint;
    listing_account.seller_token = Some(seller_token.key());
    listing_account.price_lamports = 0;
    listing_account.start_date = Some(0);
    listing_account.end_date = Some(0);
    listing_account.close_date = Some(0);
    listing_account.is_active = false;

    Ok(())
}

#[derive(Accounts)]
//check alternative
#[instruction(count: String)]
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
            count.as_ref(),
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
    pub seller_token: Option<Pubkey>,
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