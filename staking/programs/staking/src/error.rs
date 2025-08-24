use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    #[msg("Account Not Active")]
    NotActive,
    #[msg("Amount Should Be Greater Than Zero")]
    NotValidAmount,
    #[msg("Overflow Amount")]
    Overflow,
    #[msg("Underflow Amount")]
    Underflow,
    #[msg("Invalid Timestamp")]
    InvalidStamp,
    #[msg("Insufficient Balance")]
    InsufficientFunds,
    #[msg("No points to claim")]
    NoPointsToClaim,
}
