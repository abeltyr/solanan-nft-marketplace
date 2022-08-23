use {
    anchor_lang::prelude::*,
    anchor_spl::{associated_token, token},
};

use crate::{
    error::ErrorCode::{InvalidTokenAccount, MintTokenIssue, NftAlreadyListed},
    utils::create_nft_listing_pda::*,
};

pub fn create_english_auction_listing_pda_fn(
    ctx: Context<CreateEnglishAuctionListingPda>,
    count: String,
) -> Result<()> {
    msg!("English Auction nft Listing count:{}...", count);

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

    if seller_token.key() != ctx.accounts.seller_token.key() {
        return Err(InvalidTokenAccount.into());
    }

    if ctx.accounts.seller_token.amount != 1 {
        return Err(MintTokenIssue.into());
    }

    // update the listing data
    let listing_account = &mut ctx.accounts.listing_account;
    listing_account.mint = nft_listing_account.mint;
    listing_account.seller = ctx.accounts.seller.key();
    listing_account.seller_token = seller_token.key();
    listing_account.is_active = false;
    listing_account.nft_transferred = false;

    Ok(())
}

#[derive(Accounts)]
#[instruction(count: String)]
pub struct CreateEnglishAuctionListingPda<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub seller_token: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub nft_listing_account: Account<'info, NftListingData>,
    #[account(
        init,
        payer = seller,
        space = 275,
        seeds = [
            nft_listing_account.key().as_ref(),
            b"_English_Auction_",
            count.as_ref(),
        ],
        bump
    )]
    pub listing_account: Account<'info, EnglishAuctionListingData>,
    pub system_program: Program<'info, System>,
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
