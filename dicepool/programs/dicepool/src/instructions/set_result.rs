use crate::{error::PoolError, DicePlayer, DicePool};
use anchor_lang::{prelude::*, system_program};

pub fn _set_result(ctx: Context<SetResultContext>, id: u64, result: u64) -> Result<()> {
    let dice_pool_account = &mut ctx.accounts.dice_pool;
    // Only allow after pool end
    require!(
        dice_pool_account.end_time < Clock::get()?.unix_timestamp,
        PoolError::PoolNotOver
    );

    dice_pool_account.result = result;
    dice_pool_account.ended = true;

    // First pass: count winners
    let mut total_winners = 0;

    for acc_info in ctx.remaining_accounts.iter() {
        // Try to parse directly from &AccountInfo<'info>
        let dice_player: Account<DicePlayer> = Account::try_from(acc_info)?;
    
        if dice_player.target == result {
            total_winners += 1;
        }
    }

    if total_winners == 0 {
        return Ok(());
    }

    // Calculate payout
    let amount_per_winner = dice_pool_account.total_amount / total_winners;

    // Second pass: update each player
    for acc_info in ctx.remaining_accounts.iter().cloned() {
        let mut dice_player: Account<DicePlayer> = Account::try_from(&acc_info)?;
        if dice_player.target == result {
            dice_player.claimed_amount = amount_per_winner;
        } else {
            dice_player.claimed_amount = 0;
        }
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct SetResultContext<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [b"dice_pool", creator.key().as_ref(), &id.to_le_bytes()],
        bump,
        has_one = creator
    )]
    pub dice_pool: Account<'info, DicePool>,

}
