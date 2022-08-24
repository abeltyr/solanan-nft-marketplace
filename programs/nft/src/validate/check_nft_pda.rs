use anchor_lang::prelude::*;

use crate::{error::ErrorCode::NftAlreadyListed, utils::create_nft_listing_pda::*};

pub fn check_nft_listing<'info>(
    nft_listing_account: &Account<'info, NftListingData>,
) -> Result<()> {
    // check if the nft is not already listed before creating the PDA
    if nft_listing_account.active {
        return Err(NftAlreadyListed.into());
    }

    Ok(())
}
