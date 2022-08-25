use anchor_lang::prelude::*;

use crate::error::ErrorCode::{
    EndDateIsEarlierThanBeginDate, ListingAlreadyClosed, PriceIssue, SellerInvalidData,
    StartDateIsInPast,
};

pub fn check_listing_input<'info>(
    start_date: u64,
    end_date: u64,
    close_date: Option<u64>,
    price_lamports: u64,
    seller: &AccountInfo<'info>,
    listing_seller: &Pubkey,
) -> Result<()> {
    // check if the given seller is the same as the one creating the listing pda
    if listing_seller.key() != seller.key() {
        return Err(SellerInvalidData.into());
    }

    // start_date cannot be in the past
    if start_date < Clock::get().unwrap().unix_timestamp as u64 {
        return Err(StartDateIsInPast.into());
    }

    // Check if the end_date is greater than the start_date
    if start_date > end_date {
        return Err(EndDateIsEarlierThanBeginDate.into());
    }

    //check if listing is closed
    if close_date.is_some() && close_date.unwrap() > 0 {
        return Err(ListingAlreadyClosed.into());
    }

    if price_lamports <= 0 {
        return Err(PriceIssue.into());
    }

    Ok(())
}
