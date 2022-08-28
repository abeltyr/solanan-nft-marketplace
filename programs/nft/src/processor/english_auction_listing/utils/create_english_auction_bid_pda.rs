use {
    anchor_lang::prelude::*,
    anchor_spl::{associated_token, token},
};

use crate::{
    error::ErrorCode::SellerBidIssue,
    processor::{english_auction_listing::create_english_auction_listing::*, nft::mint_nft::*},
    validate::{
        check_listing_is_active::*, check_nft_owner::*, check_token_owner_and_delegation::*,
    },
};

pub fn create_english_auction_bid_pda_fn(ctx: Context<CreateEnglishAuctionBidPda>) -> Result<()> {
    msg!("English Auction Bid for Listing...",);

    let nft_listing = &ctx.accounts.nft_listing_account.to_account_info();

    let nft_listing_account = &mut ctx.accounts.nft_listing_account;

    let listing_account = &mut ctx.accounts.listing_account;

    if ctx.accounts.bidder.key() == listing_account.seller.key() {
        return Err(SellerBidIssue.into());
    }

    check_listing_is_active(
        &ctx.program_id,
        &listing_account.mint,
        listing_account.is_active,
        &nft_listing_account,
    )?;

    check_token_owner_and_delegation(&ctx.accounts.seller_token, &nft_listing.key())?;

    check_nft_owner(
        &listing_account.seller.clone(),
        &ctx.accounts.seller_token,
        nft_listing_account,
    )?;

    // fetch token account of the bidder
    let bidder_token = associated_token::get_associated_token_address(
        &ctx.accounts.bidder.key(),
        &listing_account.mint.key(),
    );

    // update the bid data
    let bid_account = &mut ctx.accounts.bid_account;
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
