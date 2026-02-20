use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub authority: Pubkey,
    pub upgrade_authority: Pubkey,
    pub total_supply: u64,
    pub stability_basket: [Pubkey; 5],  // Problem 2, 24, 26 (e.g. USDC, USDT, BTC)
    pub burn_rate: u16,                 // Problem 12 (bps)
    pub reward_share: u16,             // Problem 6, 11 (bps to stakers)
    pub bounty_share: u16,             // Problem 1 (bps for bug bounty)
    pub stability_share: u16,          // Problem 2 (bps for DSM)
    pub treasury_share: u16,           // Problem 21 (bps for offsets/ops)
    pub marketing_share: u16,          // bps for marketing
    pub team_vest_end: i64,             // Problem 27
    pub compliance_whitelist: bool,     // Problem 7, 10, 15
    pub emergency_pause: bool,          // Problem 1, 9, 23
    pub multi_oracle_consensus: bool,   // Problem 24
    pub last_price_update: i64,
    pub last_oracle_price: u64,
    pub volatility_threshold: u16,      // Problem 2 (bps)
    pub bump: u8,
}

#[account]
pub struct StakingPool {
    pub total_staked: u64,
    pub accumulated_rewards_per_share: u128,
    pub last_reward_timestamp: i64,
    pub reward_rate: u16,               // Problem 11 (bps)
    pub bump: u8,
}

#[account]
pub struct UserStake {
    pub owner: Pubkey,
    pub amount: u64,
    pub reward_tally: u128,
    pub last_stake_timestamp: i64,
    pub is_confidential: bool,          // Problem 4 (ZK mode toggle)
    pub bump: u8,
}

#[account]
pub struct Proposal {
    pub creator: Pubkey,
    pub description: String,            // Problem 13
    pub amount: u64,
    pub target: Pubkey,
    pub for_votes: u64,
    pub against_votes: u64,
    pub end_timestamp: i64,
    pub executed: bool,
    pub bump: u8,
}

#[account]
pub struct WhitelistEntry {
    pub is_allowed: bool,               // Problem 7, 15
    pub bump: u8,
}

#[account]
pub struct BlacklistEntry {
    pub is_blocked: bool,               // Problem 9, 10
    pub bump: u8,
}

#[account]
pub struct Treasury {
    pub balance: u64,
    pub signers: [Pubkey; 5],           // Problem 8, 27 (Multisig signers)
    pub bump: u8,
}
