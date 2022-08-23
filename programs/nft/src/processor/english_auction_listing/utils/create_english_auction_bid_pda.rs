use {
    anchor_lang::prelude::*,
    anchor_spl::{associated_token, token},
};

use crate::{
    error::ErrorCode::{
        InvalidTokenAccount, InvalidTokenAccountDelegation, NftNotListed, SellerBidIssue,
    },
    processor::english_auction_listing::utils::create_english_auction_listing_pda::*,
    utils::create_nft_listing_pda::*,
};

pub fn create_english_auction_bid_pda_fn(ctx: Context<CreateEnglishAuctionBidPda>) -> Result<()> {
    msg!("English Auction Bid for Listing...",);

    let nft_listing = &ctx.accounts.nft_listing_account.to_account_info();

    let nft_listing_account = &mut ctx.accounts.nft_listing_account;

    let listing_account = &mut ctx.accounts.listing_account;

    // check is the nft listing is active
    if !nft_listing_account.active || !listing_account.is_active {
        return Err(NftNotListed.into());
    }

    let bid_account = &mut ctx.accounts.bid_account;
    let listing_account = &mut ctx.accounts.listing_account;

    if ctx.accounts.bidder.key() == listing_account.seller.key() {
        return Err(SellerBidIssue.into());
    }
    // validate so that the seller can distribute

    let seller_token = associated_token::get_associated_token_address(
        &listing_account.seller.clone(),
        &listing_account.mint.clone(),
    );

    if seller_token.key() != ctx.accounts.seller_token.key() {
        return Err(InvalidTokenAccount.into());
    }

    // validate if the token is still under the owner by the token account
    if ctx.accounts.seller_token.delegate.is_none()
        || ctx.accounts.seller_token.delegate.unwrap() != nft_listing.key()
        || ctx.accounts.seller_token.delegated_amount != 100000000
        || ctx.accounts.seller_token.amount != 1
    {
        return Err(InvalidTokenAccountDelegation.into());
    }

    // fetch token account of the bidder
    let bidder_token = associated_token::get_associated_token_address(
        &ctx.accounts.bidder.key(),
        &listing_account.mint.key(),
    );

    // update the bid data
    bid_account.listing_account = ctx.accounts.listing_account.key();
    bid_account.bidder = ctx.accounts.bidder.key();
    bid_account.bidder_token = bidder_token.key();

    Ok(())
}

#[derive(Accounts)]
pub struct CreateEnglishAuctionBidPda<'info> {
    #[account(mut)]
    pub bidder: Signer<'info>,
    #[account(mut)]
    pub seller_token: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub nft_listing_account: Account<'info, NftListingData>,
    #[account(mut)]
    pub listing_account: Account<'info, EnglishAuctionListingData>,
    #[account(
        init,
        payer = bidder,
        space = 150,
        seeds = [
            listing_account.key().as_ref(),
            bidder.key().as_ref(),
        ],
        bump
    )]
    pub bid_account: Account<'info, EnglishAuctionListingBidData>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct EnglishAuctionListingBidData {
    pub listing_account: Pubkey,
    pub bidder: Pubkey,
    pub bidder_token: Pubkey,
    pub bid_price_lamports: Option<u64>,
    pub fund_deposit: Option<bool>,
    pub withdrawn_by: Option<Pubkey>,
}
