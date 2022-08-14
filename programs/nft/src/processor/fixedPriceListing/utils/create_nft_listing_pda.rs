use {anchor_lang::prelude::*, anchor_spl::token};

pub fn create_nft_listing_pda_fn(ctx: Context<CreateNftListingPda>) -> Result<()> {
    msg!("Set The Nft Listing PDA");

    let nft_listing_account = &mut ctx.accounts.nft_listing_account;
    nft_listing_account.amount = 0;
    nft_listing_account.active = false;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateNftListingPda<'info> {
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 82,
        seeds = [
            mint.key().as_ref(),
            b"_state",
        ],
        bump
    )]
    pub nft_listing_account: Account<'info, NftListingData>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct NftListingData {
    pub amount: u32,
    pub active: bool,
}
