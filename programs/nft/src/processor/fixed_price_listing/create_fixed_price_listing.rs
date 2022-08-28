use {anchor_lang::prelude::*, anchor_spl::token};

use crate::{
    error::ErrorCode::NftAlreadyListed,
    processor::nft::mint_nft::*,
    validate::{check_listing_input::*, check_nft_owner::*},
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

    let listing_account = &mut ctx.accounts.listing_account;

    // check if the nft is not already listed before creating the PDA
    if nft_listing_account.active {
        return Err(NftAlreadyListed.into());
    }

    // fetch token account of the seller and check owner
    check_nft_owner(
        &ctx.accounts.seller.key(),
        &ctx.accounts.seller_token,
        nft_listing_account,
    )?;

    //validate the listing data
    check_listing_input(start_date, end_date, price_lamports)?;

    // delegate the nft to a new a PDA
    token::approve(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Approve {
                authority: ctx.accounts.seller.to_account_info(),
                delegate: nft_listing.to_account_info(),
                to: ctx.accounts.seller_token.to_account_info(),
            },
        ),
        100000000,
    )?;

    // update the listing data
    listing_account.seller = ctx.accounts.seller.key();
    listing_account.seller_token = ctx.accounts.seller_token.key();
    listing_account.mint = nft_listing_account.mint;
    listing_account.price_lamports = price_lamports;
    listing_account.start_date = Some(start_date);
    listing_account.end_date = Some(end_date);
    listing_account.close_date = Some(0);
    listing_account.is_active = true;

    // update the nft listing data
    nft_listing_account.amount = nft_listing_account.amount + 1;
    nft_listing_account.active = true;
    nft_listing_account.listing = Some("Fixed Price".to_string());

    Ok(())
}

#[derive(Accounts)]
pub struct CreateFixedPriceListing<'info> {
    #[account(mut)]
    pub nft_listing_account: Account<'info, NftListingData>,
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub seller_token: Account<'info, token::TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
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
