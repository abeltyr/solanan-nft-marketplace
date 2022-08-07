use anchor_lang::prelude::*;

pub mod mint;

use mint::*;

declare_id!("iC2Uv1eaFAaqk72RccoJx2NupqTGYpnXY7j7b8zyhwL");

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
