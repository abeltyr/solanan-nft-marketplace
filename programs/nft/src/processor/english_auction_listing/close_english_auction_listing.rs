use {
    anchor_lang::prelude::*,
    anchor_spl::{associated_token, token},
};

use crate::{
    error::ErrorCode,
    processor::english_auction_listing::utils::create_english_auction_listing_pda::*,
    utils::create_nft_listing_pda::*,
};
pub fn close_english_auction_listing_fn(ctx: Context<CloseEnglishAuctionListing>) -> Result<()> {
    msg!("Closing The English Auction Listing...");

    let nft_listing = &ctx.accounts.nft_listing_account.to_account_info();

    let nft_listing_account = &mut ctx.accounts.nft_listing_account;
    let listing_account = &mut ctx.accounts.listing_account;

    let (_pubkey_mint, bump_seed) = Pubkey::find_program_address(
        &[listing_account.mint.key().as_ref(), b"_nft_listing_data"],
        ctx.program_id,
    );

    //check if the given nft listing data is the same
    if _pubkey_mint != nft_listing.key() {
        return Err(ErrorCode::NftListingInvalidData.into());
    }

    // check if the given seller is the same as the one provided in the listing
    // TODO:add an admin so that the admin can also close
    if listing_account.seller != ctx.accounts.seller.key() {
        return Err(ErrorCode::SellerInvalidData.into());
    }

    // fetch token account of the owner
    let seller_token = associated_token::get_associated_token_address(
        &listing_account.seller.key(),
        &listing_account.mint.key(),
    );

    // validate the given token address match with the account
    if seller_token.key() != ctx.accounts.seller_token.key() {
        return Err(ErrorCode::InvalidTokenAccount.into());
    }

    // check the given token address has access to the nft and that it has given delegation authority
    if ctx.accounts.seller_token.delegate.is_none()
        || ctx.accounts.seller_token.delegate.unwrap() != nft_listing.key()
        || ctx.accounts.seller_token.delegated_amount != 100000000
        || ctx.accounts.seller_token.amount != 1
    {
        return Err(ErrorCode::InvalidTokenAccountDelegation.into());
    }

    // check if listing is already closed
    if listing_account.close_date > Some(0)
        || !listing_account.is_active
        || listing_account.sold.is_some()
    {
        return Err(ErrorCode::ListingAlreadyClosed.into());
    }

    let mut sold = false;

    //TODO: setup a case for the nft is not available to transfer

    // if the Auction has a highest bid we use that transfer the the nft
    if listing_account.highest_bid_pda.is_some() && listing_account.highest_bidder.is_some() {
        let bidder_token = associated_token::get_associated_token_address(
            &listing_account.highest_bidder.unwrap().key(),
            &listing_account.mint.key(),
        );

        if bidder_token.key() != ctx.accounts.bidder_token.key()
            || ctx.accounts.bidder_token.key()
                != listing_account.highest_bidder_token.unwrap().key()
        {
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

    // revoke program nft id
    token::revoke(CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Revoke {
            authority: ctx.accounts.seller.to_account_info(),
            source: ctx.accounts.seller_token.to_account_info(),
        },
    ))?;

    // update the nft listing pda
    nft_listing_account.active = false;
    nft_listing_account.listing = None;

    // // close the fixed price listing pda
    listing_account.close_date = Some(Clock::get().unwrap().unix_timestamp as u64);
    listing_account.is_active = false;
    listing_account.sold = Some(sold);

    Ok(())
}

#[derive(Accounts)]
pub struct CloseEnglishAuctionListing<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub seller_token: Account<'info, token::TokenAccount>,
    #[account(mut)]
    /// CHECK: Checked under transfer
    pub bidder_token: UncheckedAccount<'info>,
    #[account(mut)]
    pub nft_listing_account: Account<'info, NftListingData>,
    #[account(mut)]
    pub listing_account: Account<'info, EnglishAuctionListingData>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
}
