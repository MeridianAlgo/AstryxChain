use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Burn, Token2022, TokenAccount, Mint};
use crate::state::Config;

pub fn bridge_out(ctx: Context<BridgeOut>, amount: u64, target_chain: u8, target_address: [u8; 32]) -> Result<()> {
    // 1. Burn tokens locally (Problem 25)
    let cpi_accounts = Burn {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token_2022::burn(cpi_context, amount)?;

    // 2. Emit Cross-Chain Message (Wormhole/LayerZero stub)
    // Problem 5 & 25: Fragmented liquidity and bridge security
    msg!("Problem 25: Bridging out {} NXS to chain {} for address {:?}", amount, target_chain, target_address);
    msg!("Security: Token burned on Solana, waiting for Wormhole VAA verification");

    Ok(())
}

#[derive(Accounts)]
pub struct BridgeOut<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"config", config.authority.as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    pub token_program: Program<'info, Token2022>,
}
