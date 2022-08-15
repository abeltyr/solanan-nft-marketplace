use {
    anchor_lang::prelude::*,
    anchor_spl::{associated_token, token},
};

use crate::{error::ErrorCode::NftAlreadyListed, utils::create_nft_listing_pda::*};

pub fn create_english_auction_listing_pda_fn(
    ctx: Context<createEnglishAuctionListingPda>,
    count: String,
) -> Result<()> {
    msg!("English Auction nft Listing count:{}...", count);

    if ctx.accounts.nft_listing_account.active {
        return Err(NftAlreadyListed.into());
    }

    // fetch token account of the seller
    let seller_token = associated_token::get_associated_token_address(
        &ctx.accounts.seller.key(),
        &ctx.accounts.mint.key(),
    );

    // // update the listing data
    let listing_account = &mut ctx.accounts.listing_account;
    listing_account.mint = ctx.accounts.mint.key();
    listing_account.seller = ctx.accounts.seller.key();
    listing_account.seller_token = Some(seller_token.key());
    listing_account.starting_price_lamports = 0;
    listing_account.start_date = Some(0);
    listing_account.end_date = Some(0);
    listing_account.close_date = Some(0);
    listing_account.is_active = false;

    Ok(())
}

#[derive(Accounts)]
#[instruction(count: String)]
pub struct createEnglishAuctionListingPda<'info> {
    #[account(mut)]
    pub bidder: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub english_listing_account: Account<'info, EnglishAuctionListingData>,
    #[account(
        init,
        payer = seller,
        space = 250,
        seeds = [
            english_listing_account.key().as_ref(),
            b"_",
            bidder.key().as_ref(),
        ],
        bump
    )]
    pub english_listing_bid_account: Account<'info, EnglishAuctionListingData>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct EnglishAuctionListingBidData {
    pub english_listing_account: Pubkey,
    pub bidder: Pubkey,
    pub bidder_token: Pubkey,
    pub bid_price_lamports: u64,
    pub bid_date: u64,
    pub fund_withdrawn: Option<bool>,
}
