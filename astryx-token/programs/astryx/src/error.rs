use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Account is not whitelisted for transfers (Problem 7, 15)")]
    NotWhitelisted,
    #[msg("Oracle price data is invalid or stale (Problem 2, 24)")]
    InvalidOracle,
    #[msg("Transfer is blocked by security protocol (Problem 1, 9, 23)")]
    Blocked,
    #[msg("Proposal has expired and can no longer be voted on (Problem 8, 13)")]
    ProposalExpired,
    #[msg("Unauthorized access or operation")]
    Unauthorized,
    #[msg("Insufficient staked amount for this operation")]
    InsufficientStakedAmount,
    #[msg("Global emergency pause is active")]
    GlobalEmergencyPause,
    #[msg("Team vesting period has not ended (Problem 27)")]
    VestNotEnded,
    #[msg("Excessive transfer amount detected (Problem 18, 19)")]
    ExcessiveAmount,
    #[msg("Blacklisted address detected (Problem 9, 10)")]
    Blacklisted,
    #[msg("Oracle price divergence exceeds threshold (Problem 24)")]
    OracleDivergence,
    #[msg("Invalid transaction memo for tax reporting (Problem 22)")]
    InvalidTaxMemo,
}
