use anchor_lang::prelude::*;

pub mod mint;

use mint::*;

declare_id!("DN4P41d49eZosBp6Y4cFsPPmwJFqhm4D759mrSgJGADm");

#[program]
pub mod nft {
    use super::*;

    pub fn mint(
        ctx: Context<MintNft>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
    ) -> Result<()> {
        mint::mint(ctx, metadata_title, metadata_symbol, metadata_uri)
    }
}
