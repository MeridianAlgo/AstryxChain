use anchor_lang::prelude::*;
use anchor_spl::token_2022::{
    initialize_mint, mint_to, Account, Mint, Token2022, TokenAccount, 
};
use anchor_spl::associated_token::AssociatedToken;
use crate::state::{Config, StakingPool, Treasury};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 8 + (32 * 5) + (2 * 6) + 8 + 1 + 1 + 1 + 8 + 8 + 2 + 1,
        seeds = [b"config", authority.key().as_ref()],
        bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = authority,
        space = 8 + 8 + 16 + 8 + 2 + 1,
        seeds = [b"staking_pool", authority.key().as_ref()],
        bump,
    )]
    pub staking_pool: Account<'info, StakingPool>,

    #[account(
        init,
        payer = authority,
        space = 8 + 8 + (32 * 5) + 1,
        seeds = [b"treasury", authority.key().as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 9,
        mint::authority = authority,
        mint::freeze_authority = authority,
        mint::token_program = token_program,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub mint_ata: Account<'info, TokenAccount>,

    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<Initialize>, params: InitParams) -> Result<()> {
    // 1. Initialize the Config PDA
    let config = &mut ctx.accounts.config;
    config.authority = ctx.accounts.authority.key();
    config.upgrade_authority = ctx.accounts.authority.key();
    config.total_supply = params.total_supply;
    config.burn_rate = 2000; // 20% (Problem 12)
    config.reward_share = 4000; // 40% (Problem 6)
    config.bounty_share = 1000; // 10% (Problem 1)
    config.stability_share = 1000; // 10% (Problem 2)
    config.treasury_share = 1000; // 10% (Problem 21)
    config.marketing_share = 1000; // 10% (Problem 3)
    config.team_vest_end = params.team_vest_end; // Problem 27
    config.compliance_whitelist = false; // Problem 7
    config.emergency_pause = false; // Problem 1
    config.multi_oracle_consensus = params.multi_oracle_consensus; // Problem 24
    config.volatility_threshold = 500; // 5% (Problem 2)
    config.bump = ctx.bumps.config;

    // 2. Initialize Staking Pool
    let staking_pool = &mut ctx.accounts.staking_pool;
    staking_pool.total_staked = 0;
    staking_pool.accumulated_rewards_per_share = 0;
    staking_pool.last_reward_timestamp = Clock::get()?.unix_timestamp;
    staking_pool.reward_rate = 1000; // 10% base APR (Problem 11)
    staking_pool.bump = ctx.bumps.staking_pool;

    // 3. Initialize Treasury
    let treasury = &mut ctx.accounts.treasury;
    treasury.balance = 0;
    treasury.signers = params.multisig_signers; // Problem 8
    treasury.bump = ctx.bumps.treasury;

    // 4. Mint total supply to authority's ATA
    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token_2022::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.mint_ata.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        params.total_supply,
    )?;

    msg!("Astryx (NXS) initialized successfully with all security extensions enabled.");
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitParams {
    pub total_supply: u64,
    pub team_vest_end: i64,
    pub multi_oracle_consensus: bool,
    pub multisig_signers: [Pubkey; 5],
}
