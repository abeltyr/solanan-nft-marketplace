use {anchor_lang::prelude::*, anchor_spl::token};

use crate::{error::ErrorCode, processor::nft_listing_pda::*};

pub fn fixed_price_nft_listing(
    ctx: Context<FixedPriceListing>,
    start_date: u64,
    end_date: u64,
) -> Result<()> {
    msg!("Start the Fixed Price listing Process");

    // Fetch the nft listing account data and validate the nft status (check if nft is already is listed or not)

    let nft_listing_account = &mut ctx.accounts.nft_listing_account;

    msg!("nft_listing_account amount: {}", nft_listing_account.amount);
    msg!(
        "nft_listing_account Active: {}",
        nft_listing_account.status == NftListingStatus::Active
    );
    msg!(
        "nft_listing_account Closed: {}",
        nft_listing_account.status == NftListingStatus::Closed
    );

    if nft_listing_account.status == NftListingStatus::Active {
        return Err(ErrorCode::NftAlreadyListed.into());
    }

    // start_date cannot be in the past
    if start_date < Clock::get().unwrap().unix_timestamp as u64 {
        return Err(ErrorCode::StartDateIsInPast.into());
    }

    // end_date should not be greater than start_date
    if start_date > end_date {
        return Err(ErrorCode::EndDateIsEarlierThanBeginDate.into());
    }

    // approve the nft
    token::approve(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Approve {
                authority: ctx.accounts.owner.to_account_info(),
                delegate: ctx.accounts.program_account.to_account_info(),
                to: ctx.accounts.owner_token_account.to_account_info(),
            },
        ),
        100000000,
    )?;

    // // update the nft listing data

    // nft_listing_account.amount = nft_listing_account.amount + 1;
    // nft_listing_account.status = NftListingStatus::Active;

    // // update the listing data
    // let listing_account = &mut ctx.accounts.listing_account;
    // listing_account.buyer = &ctx.accounts.owner.key();
    // listing_account.buyer_token = &ctx.accounts.owner_token_address.key();
    // listing_account.mint = &ctx.accounts.mint.key();
    // listing_account.price = price;
    // listing_account.start_date = start_date;
    // listing_account.end_date = end_date;
    // listing_account.close_date = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct FixedPriceListing<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub owner_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub nft_listing_account: Account<'info, NftListingData>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    #[account()]
    pub program_account: Signer<'info>,
    // #[account(
    //     init,
    //     payer = owner,
    //     space = 82,
    //     seeds = [
    //         nft_listing_account.key().as_ref(),
    //         b"_",
    //         (nft_listing_account.amount + 1).to_string().as_ref()
    //     ],
    //     bump
    // )]
    // pub listing_account: Account<'info, FixedPriceListingData>,
}

// #[account]
// #[derive(Default)]
// pub struct FixedPriceListingData {
//     pub buyer: Pubkey,
//     pub buyer_token: Pubkey,
//     pub mint: Pubkey,
//     pub seller: Option<Pubkey>,
//     pub seller_token: Option<Pubkey>,
//     pub price: u64,
//     pub start_date: u64,
//     pub end_date: u64,
//     pub close_date: u64,
//     pub sold: bool,
// }
