use anchor_lang::prelude::*;

pub fn _create_pool(
    ctx: Context<CreateDicePoolContext>,
    id: u64,
    start_time: u4,
    end_time: u64,
    capacity: u64,
    base_amount: u64,
) -> Result<()> {


    Ok(())
}

#[derive(Accounts)]
pub struct CreateDicePoolContext<'info>{
    #[account(mut)]
    pub payer : Signer<'info>,
    #[account(
        init,
        payer=payer,
        space= 8 + DicePool::INIT_SPACE,
        seeds=[],
        bump
    )]
    pub dice_pool : Account<'info,DicePool>,
    
}
