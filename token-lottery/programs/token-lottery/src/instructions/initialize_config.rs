use anchor_lang::prelude::*;

use crate::TokenLottery;

pub fn _initialize_config(
    ctx: Context<InitializeContext>,
    start_time: u64,
    end_time: u64,
    ticket_price: u64,
) -> Result<()> {
    let lottery_account = &mut ctx.accounts.token_lottery;
    lottery_account.bump = ctx.bumps.token_lottery;
    lottery_account.authority = *ctx.accounts.payer.key;
    lottery_account.end_time=end_time;
    lottery_account.start_time= start_time;
    lottery_account.ticket_price = ticket_price;
    lottery_account.lotter_pot_amount=0;
    lottery_account.randomness_account = Pubkey::default();
    lottery_account.total_tickets=0;
    lottery_account.winner_chosen=false;
    Ok(())
}
#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    #[account(
        init,
        payer=payer,
        space = 8  + TokenLottery::INIT_SPACE,
        seeds=[b"token_lottery".as_ref()],
        bump
    )]
    pub token_lottery: Account<'info, TokenLottery>,
    pub system_program: Program<'info, System>,
}
