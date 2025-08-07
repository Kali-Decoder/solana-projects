use anchor_lang::prelude::*;
// Function to create user statistics and initialize an account.
declare_id!("4HFcKqqTHWvCwqT2CL5qmbprKJwCZeXWYNB4VAQEpcYh");


pub const GAME_SEED: &str = "game";
#[program]
pub mod game {
    use super::*;

    pub fn initialize_profile(ctx:Context<InitializeProfile>,username:String) -> Result<()> {
        let game_account = &mut ctx.accounts.game_profile;
        game_account.name = username;
        game_account.xp = 0;
        game_account.level=0;
        game_account.owner = ctx.accounts.owner.key();
        game_account.bump = *ctx.bumps.get("game_profile").unwrap();
        Ok(())
    }
}


#[derive(Accounts)]
pub struct InitializeProfile<'info>{
    #[account(mut)]
    pub owner : Signer<'info>,
    #[account(
        init,
        space= 8 + GameProfile::INIT_SPACE,
        payer = owner,
        seeds = [GAME_SEED.as_bytes(),owner.key().as_ref()],
        bump,
    )]
    pub game_profile: Account<'info, GameProfile>,

}

#[account]
#[derive(InitSpace)]
pub struct GameProfile {
    pub owner: Pubkey,
    pub level: u16,
    pub xp: u16,
    #[max_len(100)]
    pub name: String,
    pub bump: u8,
}