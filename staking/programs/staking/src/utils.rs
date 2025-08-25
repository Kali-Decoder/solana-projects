use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};
use crate::constants::{POINTS_PER_SOL_PER_DAY, SECONDS_PER_DAY};
use crate::{error::StakingError, StakeAccount};
pub fn update_points(pda_account: &mut StakeAccount, current_timestamp: i64) -> Result<()> {
    let time_passed = current_timestamp
        .checked_sub(pda_account.last_update)
        .ok_or(StakingError::InvalidStamp)? as u64;

    if time_passed > 0 && pda_account.staked_amount > 0 {
        let new_points = calculate_points_earned(pda_account.staked_amount, time_passed)?;

        let current_total_points = pda_account
            .reward_amount
            .checked_add(new_points)
            .ok_or(StakingError::Overflow)?;

        msg!(
            "Current points: {}, Staked amount: {} SOL",
            current_total_points / 1_000_000,
            pda_account.staked_amount / LAMPORTS_PER_SOL
        );

    }
    Ok(())
}

pub fn calculate_points_earned(staked_amount: u64, time_passed: u64) -> Result<u64> {

    let points = (staked_amount as u128)
    .checked_mul(time_passed as u128)
    .ok_or(StakingError::Overflow)?
    .checked_mul(POINTS_PER_SOL_PER_DAY as u128)
    .ok_or(StakingError::Overflow)?
    .checked_div(LAMPORTS_PER_SOL as u128)
    .ok_or(StakingError::Overflow)?
    .checked_div(SECONDS_PER_DAY as u128)
    .ok_or(StakingError::Overflow)?;
    Ok(points as u64)
}
