use anchor_lang::prelude::*;

use crate::error::ErrorCode::{ClosingIssue, ListingAlreadyClosed};

pub fn check_listing_closing<'info>(
    closer: &AccountInfo<'info>,
    listing_account_seller: &Pubkey,
    close_date: Option<u64>,
    is_active: bool,
    sold: Option<bool>,
) -> Result<()> {
    // only the seller can close the listing
    // TODO:add an admin so that the admin can also close
    if closer.key() != listing_account_seller.key() {
        return Err(ClosingIssue.into());
    }

    // check if listing is already closed
    if close_date > Some(0) || !is_active || sold.is_some() {
        return Err(ListingAlreadyClosed.into());
    }

    Ok(())
}
