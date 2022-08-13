use {anchor_lang::prelude::*, anchor_spl::token};

use crate::{
    error::ErrorCode,
    processor::{fixed_price_listing_pda::*, nft_listing_pda::*},
};

pub fn buy_nft_fixed_price_listing_fn(ctx: Context<BuyNftFixedPriceListing>) -> Result<()> {
    msg!("Buy The Nft...");

    // transfer the fund to the account

    let nft_listing_account = &mut ctx.accounts.nft_listing_account;
    let listing_account = &mut ctx.accounts.listing_account;
    let price_lamports = listing_account.price;

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.buyer_authority.to_account_info(),
                to: ctx.accounts.program_account.to_account_info(),
            },
        ),
        price_lamports,
    )?;

    listing_account.fund_deposited = Some(true);

    /// transfer the NFT To buyer
    let seller_token_account = associated_token::get_associated_token_address(
        listing_account.seller.key(),
        &ctx.accounts.mint.key(),
    );

    let buyer_token_account = associated_token::get_associated_token_address(
        &ctx.accounts.buyer_authority.key(),
        &ctx.accounts.mint.key(),
    );

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: seller_token_account.to_account_info(),
                to: buyer_token_account.to_account_info(),
                authority: ctx.accounts.program_account.to_account_info(),
            },
        ),
        1,
    )?;

    // transfer the fund to the seller
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.program_account.to_account_info(),
                to: ctx.accounts.seller_authority.to_account_info(),
            },
        ),
        price_lamports,
    )?;

    // update the nft listing pda
    nft_listing_account.active = false;

    // close the fixed price listing pda
    listing_account.close_date = Some(Clock::get().unwrap().unix_timestamp);
    listing_account.sold = Some(true);
    listing_account.is_active = Some(false);
    listing_account.buyer = Some(&ctx.accounts.buyer_authority.key());
    listing_account.buyer_token = Some(buyer_token);
    listing_account.fund_withdrawn = Some(true);

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
    pub buyer_authority: Signer<'info>,
    #[account(mut)]
    pub seller_authority: Account<'info>,
    #[account()]
    pub program_account: Signer<'info>,
    #[account(mut)]
    pub buyer_authority: Signer<'info>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
}
