use {anchor_lang::prelude::*, anchor_spl::token};

use crate::{
    error::ErrorCode::MintTokenIssue, processor::nft_mint::utils::create_nft_listing_pda::*,
    validate::check_token_owner::*,
};

pub fn check_nft_owner<'info>(
    owner: &Pubkey,
    owner_token: &Account<'info, token::TokenAccount>,
    nft_listing_account: &Account<'info, NftListingData>,
) -> Result<()> {
    // fetch token account of the owner
    check_token_owner(owner, owner_token, &nft_listing_account.mint.key())?;

    if owner_token.amount != 1 {
        return Err(MintTokenIssue.into());
    }

    Ok(())
}
