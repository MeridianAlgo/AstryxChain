use anchor_lang::prelude::*;
use crate::state::{Config, BlacklistEntry, WhitelistEntry, UserStake};
use crate::error::ErrorCode;

pub fn toggle_compliance(ctx: Context<ToggleCompliance>, enabled: bool) -> Result<()> {
    let config = &mut ctx.accounts.config;
    if config.authority != ctx.accounts.authority.key() {
        return err!(ErrorCode::Unauthorized);
    }
    config.compliance_whitelist = enabled;
    msg!("Problem 7: Regulatory compliance whitelist toggled to {}", enabled);
    Ok(())
}

pub fn set_whitelist_status(ctx: Context<SetWhitelistStatus>, user: Pubkey, is_allowed: bool) -> Result<()> {
    let whitelist_entry = &mut ctx.accounts.whitelist_entry;
    whitelist_entry.is_allowed = is_allowed;
    whitelist_entry.bump = ctx.bumps.whitelist_entry;
    msg!("Problem 15: Whitelist status for {} set to {}", user, is_allowed);
    Ok(())
}

pub fn set_blacklist_status(ctx: Context<SetBlacklistStatus>, user: Pubkey, is_blocked: bool) -> Result<()> {
    let blacklist_entry = &mut ctx.accounts.blacklist_entry;
    blacklist_entry.is_blocked = is_blocked;
    blacklist_entry.bump = ctx.bumps.blacklist_entry;
    msg!("Problem 9: AI-flagged/Scam blacklist status for {} set to {}", user, is_blocked);
    Ok(())
}

pub fn toggle_privacy(ctx: Context<TogglePrivacy>, enabled: bool) -> Result<()> {
    let user_stake = &mut ctx.accounts.user_stake;
    user_stake.is_confidential = enabled;
    msg!("Problem 4: Optional Confidential Transfer (ZK mode) set to {}", enabled);
    Ok(())
}

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct SetWhitelistStatus<'info> {
    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + 1 + 1,
        seeds = [b"whitelist", user.as_ref()],
        bump,
    )]
    pub whitelist_entry: Account<'info, WhitelistEntry>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [b"config", authority.key().as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct SetBlacklistStatus<'info> {
    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + 1 + 1,
        seeds = [b"blacklist", user.as_ref()],
        bump,
    )]
    pub blacklist_entry: Account<'info, BlacklistEntry>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [b"config", authority.key().as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ToggleCompliance<'info> {
    #[account(mut)]
    pub config: Account<'info, Config>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct TogglePrivacy<'info> {
    #[account(
        mut,
        seeds = [b"user_stake", user.key().as_ref()],
        bump = user_stake.bump,
    )]
    pub user_stake: Account<'info, UserStake>,
    pub user: Signer<'info>,
}
