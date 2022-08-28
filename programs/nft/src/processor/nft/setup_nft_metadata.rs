use {
    anchor_lang::{prelude::*, solana_program::program},
    anchor_spl::{associated_token, token},
    mpl_token_metadata::{instruction as token_instruction, ID as TOKEN_METADATA_ID},
};

use crate::validate::check_nft_authority_relation::*;

pub fn setup_nft_metadata_fn(
    ctx: Context<SetupNftMetadata>,
    metadata_title: String,
    metadata_symbol: String,
    metadata_uri: String,
) -> Result<()> {
    msg!("Creating metadata account...");
    msg!(
        "Metadata account address: {}",
        &ctx.accounts.metadata.to_account_info().key()
    );

    let nft_listing_pda = check_nft_authority_relation(
        ctx.program_id,
        &ctx.accounts.mint.key(),
        &ctx.accounts.nft_authority_account,
    );

    let bump_seed: u8 = nft_listing_pda.unwrap().1;
    program::invoke_signed(
        &token_instruction::create_metadata_accounts_v2(
            TOKEN_METADATA_ID,
            ctx.accounts.metadata.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.nft_authority_account.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.nft_authority_account.key(),
            metadata_title,
            metadata_symbol,
            metadata_uri,
            None,
            0,
            true,
            false,
            None,
            None,
        ),
        &[
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.nft_authority_account.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        &[&[
            ctx.accounts.mint.key().as_ref(),
            b"_authority_",
            &[bump_seed],
        ]],
    )?;

    msg!("Creating master edition metadata account...");
    msg!(
        "Master edition metadata account address: {}",
        &ctx.accounts.master_edition.to_account_info().key()
    );
    program::invoke_signed(
        &token_instruction::create_master_edition_v3(
            TOKEN_METADATA_ID,
            ctx.accounts.master_edition.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.nft_authority_account.key(),
            ctx.accounts.nft_authority_account.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint_authority.key(),
            Some(0),
        ),
        &[
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.nft_authority_account.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        &[&[
            ctx.accounts.mint.key().as_ref(),
            b"_authority_",
            &[bump_seed],
        ]],
    )?;

    msg!("Token mint process completed successfully.");

    Ok(())
}

#[derive(Accounts)]
pub struct SetupNftMetadata<'info> {
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint: Signer<'info>,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK:
    pub nft_authority_account: UncheckedAccount<'info>,
}
