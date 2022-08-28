use anchor_lang::prelude::*;

use crate::error::ErrorCode::NftAuthorityInvalidData;

pub fn check_nft_authority_relation<'info>(
    program_id: &Pubkey,
    mint: &Pubkey,
    nft_authority_account: &AccountInfo<'info>,
) -> Result<(Pubkey, u8)> {
    let (_pubkey, _seed) =
        Pubkey::find_program_address(&[mint.key().as_ref(), b"_authority_"], program_id);

    // check if the given nft authority data is the same
    if _pubkey != nft_authority_account.key() {
        return Err(NftAuthorityInvalidData.into());
    }

    Ok((_pubkey, _seed))
}
