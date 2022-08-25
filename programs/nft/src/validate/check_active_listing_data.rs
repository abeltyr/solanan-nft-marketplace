use {anchor_lang::prelude::*, anchor_spl::token};

use crate::{
    error::ErrorCode::{ListingAlreadyClosed, ListingEnded, ListingNotActivate, ListingNotStarted},
    validate::check_token_owner_and_delegation::*,
};

pub fn check_active_listing_data<'info>(
    start_date: Option<u64>,
    end_date: Option<u64>,
    close_date: Option<u64>,
    price_lamports: u64,
    sold: Option<bool>,
    nft_listing: &AccountInfo<'info>,
    seller_token: &Account<'info, token::TokenAccount>,
) -> Result<()> {
    // check is the listing is set
    if start_date.is_none()
        || start_date.is_some() && start_date.unwrap() == 0
        || end_date.is_none()
        || end_date.is_some() && end_date.unwrap() == 0
        || price_lamports == 0
    {
        return Err(ListingNotActivate.into());
    }

    //check if listing is closed
    if close_date.is_some() && close_date.unwrap() > 0 || sold.is_some() {
        return Err(ListingAlreadyClosed.into());
    }

    let current_time = Clock::get().unwrap().unix_timestamp as u64;

    // check if the start date has passed
    if start_date.unwrap() > current_time {
        return Err(ListingNotStarted.into());
    }

    // check if the end date has not passed
    if end_date.unwrap() < current_time {
        return Err(ListingEnded.into());
    }

    check_token_owner_and_delegation(&seller_token, &nft_listing.key())?;

    Ok(())
}
