use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Transfer, Token2022, TokenAccount};
use crate::state::{Proposal, Config, Treasury, UserStake};
use crate::error::ErrorCode;

pub fn create_proposal(ctx: Context<CreateProposal>, description: String, amount: u64, target: Pubkey, duration: i64) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    proposal.creator = ctx.accounts.creator.key();
    proposal.description = description;
    proposal.amount = amount;
    proposal.target = target;
    proposal.for_votes = 0;
    proposal.against_votes = 0;
    proposal.end_timestamp = Clock::get()?.unix_timestamp + duration;
    proposal.executed = false;
    proposal.bump = ctx.bumps.proposal;

    msg!("Problem 13: New Astryx DAO Proposal created for treasury spend");
    Ok(())
}

pub fn vote(ctx: Context<Vote>, approve: bool) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let user_stake = &ctx.accounts.user_stake;
    
    if Clock::get()?.unix_timestamp > proposal.end_timestamp {
        return err!(ErrorCode::ProposalExpired);
    }

    // Problem 13: Voting weight based on staked amount
    if approve {
        proposal.for_votes += user_stake.amount;
    } else {
        proposal.against_votes += user_stake.amount;
    }

    msg!("Problem 8: Weighted vote cast on-chain for Astryx DAO");
    Ok(())
}

pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let treasury = &ctx.accounts.treasury;

    if Clock::get()?.unix_timestamp <= proposal.end_timestamp {
        return err!(ErrorCode::Unauthorized); // Proposal still active
    }

    if proposal.for_votes > proposal.against_votes && !proposal.executed {
        // Problem 13: Execute treasury transfer
        let seeds = &[
            b"treasury",
            ctx.accounts.config.authority.as_ref(),
            &[treasury.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.treasury_token_account.to_account_info(),
            to: ctx.accounts.target_token_account.to_account_info(),
            authority: treasury.to_account_info(),
        };
        let cpi_context = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, signer);
        token_2022::transfer(cpi_context, proposal.amount)?;

        proposal.executed = true;
        msg!("Problem 8: Progressive decentralization: Proposal executed by Astryx DAO");
    }

    Ok(())
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + 32 + 200 + 8 + 32 + 8 + 8 + 8 + 1 + 1,
        seeds = [b"proposal", creator.key().as_ref(), &Clock::get()?.unix_timestamp.to_le_bytes()],
        bump,
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub voter: Signer<'info>,

    #[account(
        seeds = [b"user_stake", voter.key().as_ref()],
        bump = user_stake.bump,
    )]
    pub user_stake: Account<'info, UserStake>,
}

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,

    #[account(
        seeds = [b"treasury", config.authority.as_ref()],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub target_token_account: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"config", config.authority.as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    pub token_program: Program<'info, Token2022>,
}
