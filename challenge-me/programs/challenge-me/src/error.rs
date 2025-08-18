use anchor_lang::prelude::*;

#[error_code]
pub enum ChallengeError {
    #[msg("Challenge is already completed.")]
    AlreadyCompleted,

    #[msg("Challenge progress reset due to missed day.")]
    ChallengeReset,
}
