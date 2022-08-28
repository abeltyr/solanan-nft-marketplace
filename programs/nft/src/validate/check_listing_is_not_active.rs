use anchor_lang::prelude::*;

use crate::{
    error::ErrorCode, processor::nft::mint_nft::*, validate::check_nft_listing_relation::*,
};

pub fn check_listing_is_not_active<'info>(
    program_id: &Pubkey,
    mint: &Pubkey,
    is_active: bool,
    nft_listing_account: &Account<'info, NftListingData>,
) -> Result<(Pubkey, u8)> {
    let nft_listing_pda = check_nft_listing_relation(program_id, mint, nft_listing_account);
    let mut _pubkey: Pubkey = nft_listing_account.key();
    let mut _seed: u8 = 0;
    match nft_listing_pda {
        Ok(data) => {
            _pubkey = data.0;
            _seed = data.1;
        }
        Err(e) => return Err(e),
    }

    // check is the nft is not listed
    if nft_listing_account.active || is_active {
        return Err(ErrorCode::NftAlreadyListed.into());
    }

    Ok((_pubkey, _seed))
}
