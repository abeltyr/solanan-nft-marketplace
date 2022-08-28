use {anchor_lang::prelude::*, anchor_spl::token};

use crate::{
    error::ErrorCode,
    processor::{
        english_auction_listing::utils::create_english_auction_listing_pda::*, nft::mint_nft::*,
    },
    validate::{
        check_listing_closing::*, check_nft_listing_relation::*, check_token_owner::*,
        check_token_owner_and_delegation::*,
    },
};
pub fn close_english_auction_listing_fn(ctx: Context<CloseEnglishAuctionListing>) -> Result<()> {
    msg!("Closing The English Auction Listing...");

    let nft_listing = &ctx.accounts.nft_listing_account.to_account_info();
    let nft_authority = &ctx.accounts.nft_authority_account.to_account_info();

    let nft_listing_account = &mut ctx.accounts.nft_listing_account;
    let listing_account = &mut ctx.accounts.listing_account;

    let nft_listing_pda =
        check_nft_listing_relation(&ctx.program_id, &listing_account.mint, &nft_listing_account)?;

    let bump_seed = nft_listing_pda.1;

    check_token_owner(
        &listing_account.seller.clone(),
        &ctx.accounts.seller_token,
        &nft_listing_account.mint.key(),
    )?;

    check_listing_closing(
        &ctx.accounts.closer,
        &listing_account.seller.clone(),
        listing_account.close_date,
        listing_account.is_active,
        listing_account.sold,
    )?;

    let mut sold = false;

    //TODO: setup a case to close if the nft is not available to transfer
    // if the Auction has a highest bid we use that transfer the the nft
    if listing_account.highest_bid_pda.is_some() && listing_account.highest_bidder.is_some() {
        check_token_owner_and_delegation(&ctx.accounts.seller_token, &nft_listing.key())?;

        msg!("In Here");
        //check bidder token match
        check_token_owner(
            &listing_account.highest_bidder.unwrap().key(),
            &ctx.accounts.bidder_token,
            &listing_account.mint.key(),
        )?;

        if listing_account.highest_bidder_token.unwrap().key() != ctx.accounts.bidder_token.key() {
            return Err(ErrorCode::InvalidTokenAccount.into());
        }
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.seller_token.to_account_info(),
                    to: ctx.accounts.bidder_token.to_account_info(),
                    authority: nft_listing.to_account_info(),
                },
                &[&[
                    listing_account.mint.key().as_ref(),
                    b"_nft_listing_data",
                    &[bump_seed],
                ]],
            ),
            1,
        )?;

        listing_account.nft_transferred = true;
        sold = true;
    }
    msg!("nft_authority {:?}", nft_authority);
    msg!(
        "nft_listing_account.mint.key() {:?}",
        nft_listing_account.mint.key()
    );

    if ctx.accounts.closer.key() == listing_account.seller {
        // revoke program token id
        token::revoke(CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Revoke {
                authority: ctx.accounts.closer.to_account_info(),
                source: ctx.accounts.seller_token.to_account_info(),
            },
        ))?;
    }

    // update the nft listing pda
    nft_listing_account.active = false;
    nft_listing_account.listing = None;

    // close the fixed price listing pda
    listing_account.close_date = Some(Clock::get().unwrap().unix_timestamp as u64);
    listing_account.is_active = false;
    listing_account.sold = Some(sold);

    Ok(())
}

#[derive(Accounts)]
pub struct CloseEnglishAuctionListing<'info> {
    #[account(mut)]
    pub closer: Signer<'info>,
    #[account(mut)]
    pub seller_token: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub bidder_token: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub nft_listing_account: Account<'info, NftListingData>,
    #[account(mut)]
    pub listing_account: Account<'info, EnglishAuctionListingData>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    /// CHECK:
    pub nft_authority_account: UncheckedAccount<'info>,
}
