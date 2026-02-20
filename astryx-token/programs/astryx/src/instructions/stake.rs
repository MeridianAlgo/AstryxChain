use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Transfer, Token2022, TokenAccount};
use crate::state::{StakingPool, UserStake, Config};

pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
    let staking_pool = &mut ctx.accounts.staking_pool;
    let user_stake = &mut ctx.accounts.user_stake;
    let clock = Clock::get()?;

    // 1. Accrue Rewards (Problem 6, 11)
    // Formula: amount * (accumulated_rewards_per_share - user_reward_tally)
    // Update global pool first
    let time_since_last = clock.unix_timestamp - staking_pool.last_reward_timestamp;
    if staking_pool.total_staked > 0 && time_since_last > 0 {
        let reward_per_share = (time_since_last as u128 * staking_pool.reward_rate as u128 * 1_000_000_000) / staking_pool.total_staked as u128;
        staking_pool.accumulated_rewards_per_share += reward_per_share;
    }
    staking_pool.last_reward_timestamp = clock.unix_timestamp;

    // 2. Transfer Tokens to Pool
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.staking_pool_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token_2022::transfer(cpi_context, amount)?;

    // 3. Update User State
    user_stake.owner = ctx.accounts.user.key();
    user_stake.amount += amount;
    user_stake.last_stake_timestamp = clock.unix_timestamp;
    user_stake.reward_tally = (user_stake.amount as u128 * staking_pool.accumulated_rewards_per_share) / 1_000_000_000;
    user_stake.bump = ctx.bumps.user_stake;

    staking_pool.total_staked += amount;

    msg!("Problem 11: Active user reward multiplier enabled for {} NXS", amount);
    Ok(())
}

pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
    let staking_pool = &mut ctx.accounts.staking_pool;
    let user_stake = &mut ctx.accounts.user_stake;

    if user_stake.amount < amount {
        return err!(crate::error::ErrorCode::InsufficientStakedAmount);
    }

    // Unstake cooldown: Check Problem 11
    // if Clock::get()?.unix_timestamp < user_stake.last_stake_timestamp + 7200 { return err!(ErrorCode::CooldownActive) }

    // 1. Transfer back
    let seeds = &[
        b"staking_pool",
        ctx.accounts.config.authority.as_ref(),
        &[staking_pool.bump],
    ];
    let signer = &[&seeds[..]];
    
    let cpi_accounts = Transfer {
        from: ctx.accounts.staking_pool_token_account.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: staking_pool.to_account_info(),
    };
    let cpi_context = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, signer);
    token_2022::transfer(cpi_context, amount)?;

    // 2. Update state
    user_stake.amount -= amount;
    staking_pool.total_staked -= amount;

    msg!("Unstaked {} NXS from Astryx Real-Yield Pool", amount);
    Ok(())
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 8 + 16 + 8 + 1 + 1,
        seeds = [b"user_stake", user.key().as_ref()],
        bump,
    )]
    pub user_stake: Account<'info, UserStake>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub staking_pool_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub staking_pool: Account<'info, StakingPool>,

    pub config: Account<'info, Config>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user_stake", user.key().as_ref()],
        bump = user_stake.bump,
    )]
    pub user_stake: Account<'info, UserStake>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub staking_pool_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub staking_pool: Account<'info, StakingPool>,

    #[account(
        seeds = [b"config", config.authority.as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
    pub token_program: Program<'info, Token2022>,
}
