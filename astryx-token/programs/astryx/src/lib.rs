use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod error;

use instructions::initialize::*;
use instructions::transfer_hook::*;
use instructions::stake::*;
use instructions::governance::*;
use instructions::user_controls::*;
use instructions::bridge::*;

declare_id!("AstryxProgramIdHere111111111111111111111111111");

#[program]
pub mod astryx {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: InitParams) -> Result<()> {
        instructions::initialize::handler(ctx, params)
    }

    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        instructions::transfer_hook::handler(ctx, amount)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        instructions::stake::stake(ctx, amount)
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        instructions::stake::unstake(ctx, amount)
    }

    pub fn create_proposal(ctx: Context<CreateProposal>, description: String, amount: u64, target: Pubkey, duration: i64) -> Result<()> {
        instructions::governance::create_proposal(ctx, description, amount, target, duration)
    }

    pub fn vote(ctx: Context<Vote>, approve: bool) -> Result<()> {
        instructions::governance::vote(ctx, approve)
    }

    pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
        instructions::governance::execute_proposal(ctx)
    }

    pub fn toggle_compliance(ctx: Context<ToggleCompliance>, enabled: bool) -> Result<()> {
        instructions::user_controls::toggle_compliance(ctx, enabled)
    }

    pub fn set_whitelist_status(ctx: Context<SetWhitelistStatus>, user: Pubkey, is_allowed: bool) -> Result<()> {
        instructions::user_controls::set_whitelist_status(ctx, user, is_allowed)
    }

    pub fn set_blacklist_status(ctx: Context<SetBlacklistStatus>, user: Pubkey, is_blocked: bool) -> Result<()> {
        instructions::user_controls::set_blacklist_status(ctx, user, is_blocked)
    }

    pub fn toggle_privacy(ctx: Context<TogglePrivacy>, enabled: bool) -> Result<()> {
        instructions::user_controls::toggle_privacy(ctx, enabled)
    }

    pub fn bridge_out(ctx: Context<BridgeOut>, amount: u64, target_chain: u8, target_address: [u8; 32]) -> Result<()> {
        instructions::bridge::bridge_out(ctx, amount, target_chain, target_address)
    }
}

// Fallback for Transfer Hook Interface
#[cfg(not(feature = "no-entrypoint"))]
#[entrypoint]
pub fn fallback(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Basic routing logic for SPL interface extensions
    Ok(())
}
