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
        game_account.bump = *ctx.bumps.get("game_profile").ok_or(GameError::BumpNotFound)?;
        Ok(())
    }

    pub fn save_game(ctx:Context<UpdateProfile>,xp:u16,level:u16) -> Result<()> {
        let game_account = &mut ctx.accounts.game_profile;
        if ctx.accounts.owner.key() != game_account.owner {
            return Err(GameError::Unauthorized.into());
        }
        game_account.xp += xp;
        game_account.level = level;

        Ok(())
    }
}


#[derive(Accounts)]
pub struct UpdateProfile<'info>{
    #[account(mut)]
    pub owner : Signer<'info>,
    #[account(mut)]
    pub game_profile: Account<'info, GameProfile>
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


#[error_code]
pub enum GameError {
    #[msg("Unauthorized action")]
    Unauthorized,
    #[msg("Bump not found")]
    BumpNotFound,
}
