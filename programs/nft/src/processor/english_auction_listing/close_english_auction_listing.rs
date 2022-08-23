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

    // fetch token account of the owner
    let owner_token_account = associated_token::get_associated_token_address(
        &listing_account.seller.key(),
        &listing_account.mint.key(),
    );

    if owner_token_account.key() != ctx.accounts.owner_token_account.key() {
        return Err(ErrorCode::InvalidTokenAccount.into());
    }

    // check if listing is already closed
    if listing_account.close_date > Some(0)
        || !listing_account.is_active
        || listing_account.sold.is_some()
        || listing_account.fund_withdrawn.is_some()
    {
        return Err(ErrorCode::ListingAlreadyClosed.into());
    }

    let mut sold = false;

    if listing_account.highest_bid_pda.is_some() && listing_account.highest_bidder.is_some() {
        let bidder_token_account = associated_token::get_associated_token_address(
            &listing_account.highest_bidder.unwrap().key(),
            &listing_account.mint.key(),
        );
        if bidder_token_account.key() != ctx.accounts.bidder_token_account.key()
            || ctx.accounts.bidder_token_account.key()
                != listing_account.highest_bidder_token.unwrap().key()
        {
            return Err(ErrorCode::InvalidTokenAccount.into());
        }
        let (_pubkey_mint, bump_seed) = Pubkey::find_program_address(
            &[ctx.accounts.mint.key().as_ref(), b"_state"],
            ctx.program_id,
        );
        msg!(
            "_pubkey_mint {} nft_listing {:?}",
            _pubkey_mint,
            nft_listing,
        );

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.owner_token_account.to_account_info(),
                    to: ctx.accounts.bidder_token_account.to_account_info(),
                    authority: nft_listing.to_account_info(),
                },
                &[&[ctx.accounts.mint.key().as_ref(), b"_state", &[bump_seed]]],
            ),
            1,
        )?;

        sold = true;
    }

    // revoke program nft id
    token::revoke(CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Revoke {
            authority: ctx.accounts.owner.to_account_info(),
            source: ctx.accounts.owner_token_account.to_account_info(),
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
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub owner_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub bidder_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub nft_listing_account: Account<'info, NftListingData>,
    #[account(mut)]
    pub listing_account: Account<'info, EnglishAuctionListingData>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
}
