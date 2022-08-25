use {
    anchor_lang::prelude::*,
    anchor_spl::{associated_token, token},
};

use crate::error::ErrorCode::InvalidTokenAccount;

pub fn check_token_owner<'info>(
    owner: &Pubkey,
    owner_token: &Account<'info, token::TokenAccount>,
    mint: &Pubkey,
) -> Result<()> {
    // fetch token account of the owner
    let fetch_owner_token =
        associated_token::get_associated_token_address(&owner.key(), &mint.key());

    // validate the owner token and given token
    if owner_token.key() != fetch_owner_token.key() {
        return Err(InvalidTokenAccount.into());
    }

    Ok(())
}
