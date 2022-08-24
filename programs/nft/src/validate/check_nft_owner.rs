use {
    anchor_lang::prelude::*,
    anchor_spl::{associated_token, token},
};

use crate::{
    error::ErrorCode::{InvalidTokenAccount, MintTokenIssue},
    utils::create_nft_listing_pda::*,
};

pub fn check_nft_owner<'info>(
    owner: &Pubkey,
    owner_token: &Account<'info, token::TokenAccount>,
    nft_listing_account: &Account<'info, NftListingData>,
) -> Result<Pubkey> {
    // fetch token account of the owner
    let fetch_owner_token = associated_token::get_associated_token_address(
        &owner.key(),
        &nft_listing_account.mint.key(),
    );

    if owner_token.key() != fetch_owner_token.key() {
        return Err(InvalidTokenAccount.into());
    }

    if owner_token.amount != 1 {
        return Err(MintTokenIssue.into());
    }

    Ok(fetch_owner_token)
}
