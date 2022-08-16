use {
    anchor_lang::prelude::*,
    anchor_spl::{associated_token, token},
};

use crate::{error::ErrorCode::NftAlreadyListed, utils::create_english_auction_listing_pda::*};

pub fn create_english_auction_bid_pda_fn(ctx: Context<CreateEnglishAuctionBidPda>) -> Result<()> {
    msg!("English Auction Bid for Listing...",);

    if !ctx.accounts.english_listing_account.is_active {
        return Err(NftAlreadyListed.into());
    }

    // fetch token account of the bidder
    let bidder_token = associated_token::get_associated_token_address(
        &ctx.accounts.bidder.key(),
        &ctx.accounts.mint.key(),
    );

    // // update the bid data
    let english_listing_bid_account = &mut ctx.accounts.english_listing_bid_account;
    english_listing_bid_account.english_listing_account =
        ctx.accounts.english_listing_account.key();
    english_listing_bid_account.bidder = ctx.accounts.bidder.key();
    english_listing_bid_account.bidder_token = bidder_token.key();

    Ok(())
}

#[derive(Accounts)]
pub struct CreateEnglishAuctionBidPda<'info> {
    #[account(mut)]
    pub bidder: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub english_listing_account: Account<'info, EnglishAuctionListingData>,
    #[account(
        init,
        payer = bidder,
        space = 150,
        seeds = [
            english_listing_account.key().as_ref(),
            b"_",
            bidder.key().as_ref(),
        ],
        bump
    )]
    pub english_listing_bid_account: Account<'info, EnglishAuctionListingBidData>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct EnglishAuctionListingBidData {
    pub english_listing_account: Pubkey,
    pub bidder: Pubkey,
    pub bidder_token: Pubkey,
    pub bid_price_lamports: Option<u64>,
    pub bid_date: Option<u64>,
    pub fund_deposit: Option<bool>,
    pub fund_withdrawn: Option<bool>,
}
