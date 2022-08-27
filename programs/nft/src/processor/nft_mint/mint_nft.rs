use {
    anchor_lang::{prelude::*, system_program},
    anchor_spl::{associated_token, token},
};

use crate::validate::check_nft_authority_relation::*;

pub fn mint_nft_fn(ctx: Context<MintNft>) -> Result<()> {
    msg!("Creating mint account...");
    msg!("Mint: {}", &ctx.accounts.mint.key());

    let nft_listing_pda = check_nft_authority_relation(
        ctx.program_id,
        &ctx.accounts.mint.key(),
        &ctx.accounts.nft_authority_account,
    );

    let bump_seed: u8 = nft_listing_pda.unwrap().1;

    system_program::create_account(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            system_program::CreateAccount {
                from: ctx.accounts.mint_authority.to_account_info(),
                to: ctx.accounts.mint.to_account_info(),
            },
        ),
        10000000,
        82,
        &ctx.accounts.token_program.key(),
    )?;

    msg!("Initializing mint account...");
    msg!("Mint: {}", &ctx.accounts.mint.key());
    token::initialize_mint(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::InitializeMint {
                mint: ctx.accounts.mint.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        0,
        &ctx.accounts.nft_authority_account.key(),
        Some(&ctx.accounts.nft_authority_account.key()),
    )?;

    msg!("Creating token account...");
    msg!("Token Address: {}", &ctx.accounts.token_account.key());
    associated_token::create(CpiContext::new(
        ctx.accounts.associated_token_program.to_account_info(),
        associated_token::Create {
            payer: ctx.accounts.mint_authority.to_account_info(),
            associated_token: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
    ))?;

    msg!("Minting token to token account...");
    msg!("Mint: {}", &ctx.accounts.mint.to_account_info().key());
    msg!("Token Address: {}", &ctx.accounts.token_account.key());
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.nft_authority_account.to_account_info(),
            },
            &[&[
                ctx.accounts.mint.key().as_ref(),
                b"_authority_",
                &[bump_seed],
            ]],
        ),
        1,
    )?;

    msg!("Token mint process completed successfully.");

    Ok(())
}

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub mint: Signer<'info>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    #[account(
        init,
        payer = mint_authority,
        space = 0,
        seeds = [
            mint.key().as_ref(),
            b"_authority_",
        ],
        bump
    )]
    /// CHECK:
    pub nft_authority_account: UncheckedAccount<'info>,
}
