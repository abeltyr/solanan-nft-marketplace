use anchor_lang::prelude::*;

use crate::error::ErrorCode::{EndDateIsEarlierThanBeginDate, PriceIssue, StartDateIsInPast};

pub fn check_listing_input<'info>(
    start_date: u64,
    end_date: u64,
    price_lamports: u64,
) -> Result<()> {
    // start_date cannot be in the past
    if start_date < Clock::get().unwrap().unix_timestamp as u64 {
        return Err(StartDateIsInPast.into());
    }

    // Check if the end_date is greater than the start_date
    if start_date > end_date {
        return Err(EndDateIsEarlierThanBeginDate.into());
    }

    if price_lamports <= 0 {
        return Err(PriceIssue.into());
    }

    Ok(())
}
