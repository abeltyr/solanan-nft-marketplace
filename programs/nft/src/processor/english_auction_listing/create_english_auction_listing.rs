use {anchor_lang::prelude::*, anchor_spl::token};

use crate::{
    error::ErrorCode::NftAlreadyListed,
    processor::nft::mint_nft::*,
    validate::{check_listing_input::*, check_nft_owner::*},
};

pub fn create_english_auction_listing_fn(
    ctx: Context<CreateEnglishAuctionListing>,
    start_date: u64,
    end_date: u64,
    starting_price_lamports: u64,
) -> Result<()> {
    msg!("Start the English Auction listing Process");

    // Fetch the nft listing account data and validate the nft status (check if nft is already is listed or not)

    let nft_listing = &ctx.accounts.nft_listing_account.to_account_info();

    let nft_listing_account = &mut ctx.accounts.nft_listing_account;

    // check if the nft is not already listed before creating the PDA
    if nft_listing_account.active {
        return Err(NftAlreadyListed.into());
    }

    let listing_account = &mut ctx.accounts.listing_account;

    // fetch token account of the seller and check owner
    check_nft_owner(
        &ctx.accounts.seller.key(),
        &ctx.accounts.seller_token,
        nft_listing_account,
    )?;

    //validate the listing data
    check_listing_input(start_date, end_date, starting_price_lamports)?;

    // approve the nft
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
    listing_account.mint = nft_listing_account.mint;
    listing_account.seller = ctx.accounts.seller.key();
    listing_account.seller_token = ctx.accounts.seller_token.key();
    listing_account.nft_transferred = false;
    listing_account.starting_price_lamports = starting_price_lamports;
    listing_account.start_date = Some(start_date);
    listing_account.end_date = Some(end_date);
    listing_account.close_date = Some(0);
    listing_account.highest_bid_lamports = Some(0);
    listing_account.is_active = true;

    // update the nft listing data
    nft_listing_account.amount = nft_listing_account.amount + 1;
    nft_listing_account.active = true;
    nft_listing_account.listing = Some("English Auction".to_string());
    Ok(())
}

#[derive(Accounts)]
pub struct CreateEnglishAuctionListing<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub seller_token: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub nft_listing_account: Account<'info, NftListingData>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    #[account(
        init,
        payer = seller,
        space = 275,
        seeds = [
            nft_listing_account.key().as_ref(),
            b"_English_Auction_",
            nft_listing_account.amount.to_string().as_ref(),
        ],
        bump
    )]
    pub listing_account: Account<'info, EnglishAuctionListingData>,
}

#[account]
#[derive(Default)]
pub struct EnglishAuctionListingData {
    pub mint: Pubkey,
    pub seller: Pubkey,
    pub is_active: bool,
    pub seller_token: Pubkey,
    pub starting_price_lamports: u64,
    pub start_date: Option<u64>,
    pub end_date: Option<u64>,
    pub close_date: Option<u64>,
    pub highest_bidder: Option<Pubkey>,
    pub highest_bidder_token: Option<Pubkey>,
    pub highest_bid_pda: Option<Pubkey>,
    pub highest_bid_lamports: Option<u64>,
    pub sold: Option<bool>,
    pub nft_transferred: bool,
}
