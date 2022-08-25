use {anchor_lang::prelude::*, anchor_spl::token};

use crate::error::ErrorCode::InvalidTokenAccountDelegation;

pub fn check_token_owner_and_delegation<'info>(
    seller_token: &Account<'info, token::TokenAccount>,
    nft_listing: &Pubkey,
) -> Result<()> {
    // validate if the token is still under the owner by the token account and the nft delegation is set
    if seller_token.delegate.is_none()
        || seller_token.delegate.unwrap() != nft_listing.key()
        || seller_token.delegated_amount != 100000000
        || seller_token.amount != 1
    {
        return Err(InvalidTokenAccountDelegation.into());
    }
    Ok(())
}
