use {
    anchor_lang::{prelude::*, system_program},
    anchor_spl::{associated_token, token},
};

use crate::{
    error::ErrorCode,
    processor::{create_fixed_price_listing_pda::*, create_nft_listing_pda::*},
};

pub fn buy_nft_fixed_price_listing_fn(ctx: Context<BuyNftFixedPriceListing>) -> Result<()> {
    msg!("Buy The Nft...");

    // validate the pda data

    let nft_listing_account = &mut ctx.accounts.nft_listing_account;
    let listing_account = &mut ctx.accounts.listing_account;

    if !nft_listing_account.active {
        return Err(ErrorCode::NftNotListed.into());
    }

    if listing_account.price_lamports == 0 {
        return Err(ErrorCode::ListingPriceNotSet.into());
    }

    if listing_account.close_date > Some(0)
        || listing_account.sold.is_some()
        || listing_account.fund_withdrawn.is_some()
        || listing_account.fund_deposited.is_some()
    {
        return Err(ErrorCode::ListingAlreadyClosed.into());
    }

    if listing_account.start_date == Some(0)
        || listing_account.end_date == Some(0)
        || !listing_account.is_active
    {
        return Err(ErrorCode::ListingNotActivate.into());
    }

    // // transfer the fund to the account

    // system_program::transfer(
    //     CpiContext::new(
    //         ctx.accounts.system_program.to_account_info(),
    //         system_program::Transfer {
    //             from: ctx.accounts.buyer.to_account_info(),
    //             to: ctx.accounts.program_account.to_account_info(),
    //         },
    //     ),
    //     listing_account.price_lamports,
    // )?;

    // listing_account.fund_deposited = Some(true);
    // listing_account.buyer = Some(ctx.accounts.buyer.key());

    // // transfer the NFT To buyer

    // let seller_token_account = associated_token::get_associated_token_address(
    //     &listing_account.seller.key(),
    //     &ctx.accounts.mint.key(),
    // );

    // let buyer_token_account = associated_token::get_associated_token_address(
    //     &ctx.accounts.buyer.key(),
    //     &ctx.accounts.mint.key(),
    // );

    // if seller_token_account.key() != ctx.accounts.seller_token_account.key() {
    //     return Err(ErrorCode::InvalidTokenAccount.into());
    // }

    // if buyer_token_account.key() != ctx.accounts.buyer_token_account.key() {
    //     return Err(ErrorCode::InvalidTokenAccount.into());
    // }

    // token::transfer(
    //     CpiContext::new(
    //         ctx.accounts.token_program.to_account_info(),
    //         token::Transfer {
    //             from: ctx.accounts.seller_token_account.to_account_info(),
    //             to: ctx.accounts.buyer_token_account.to_account_info(),
    //             authority: ctx.accounts.program_account.to_account_info(),
    //         },
    //     ),
    //     1,
    // )?;

    // // transfer the fund to the seller
    // system_program::transfer(
    //     CpiContext::new(
    //         ctx.accounts.system_program.to_account_info(),
    //         system_program::Transfer {
    //             from: ctx.accounts.program_account.to_account_info(),
    //             to: ctx.accounts.seller.to_account_info(),
    //         },
    //     ),
    //     listing_account.price_lamports,
    // )?;

    // // update the nft listing pda
    // nft_listing_account.active = false;

    // // close the fixed price listing pda
    // let close_date: u64 = Clock::get().unwrap().unix_timestamp as u64;
    // listing_account.close_date = Some(close_date);
    // listing_account.sold = Some(true);
    // listing_account.is_active = false;
    // listing_account.buyer_token = Some(buyer_token_account.key());
    // listing_account.fund_withdrawn = Some(true);

    Ok(())
}

#[derive(Accounts)]
pub struct BuyNftFixedPriceListing<'info> {
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub nft_listing_account: Account<'info, NftListingData>,
    #[account(mut)]
    pub listing_account: Account<'info, FixedPriceListingData>,
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub seller_token_account: Account<'info, token::TokenAccount>,
    #[account()]
    pub program_account: Signer<'info>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, token::TokenAccount>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
}
