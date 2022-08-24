use anchor_lang::prelude::*;

use crate::{error::ErrorCode, utils::create_nft_listing_pda::*};

pub fn check_nft_listing_match<'info>(
    program_id: &Pubkey,
    mint: &Pubkey,
    is_active: bool,
    nft_listing_account: &Account<'info, NftListingData>,
) -> Result<(Pubkey, u8)> {
    let (_pubkey, _seed) =
        Pubkey::find_program_address(&[mint.key().as_ref(), b"_nft_listing_data"], program_id);

    // check if the given nft listing data is the same
    if _pubkey != nft_listing_account.key() {
        return Err(ErrorCode::NftListingInvalidData.into());
    }

    if nft_listing_account.active || is_active {
        return Err(ErrorCode::NftAlreadyListed.into());
    }

    Ok((_pubkey, _seed))
}
