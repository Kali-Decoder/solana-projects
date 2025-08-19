use anchor_lang::prelude::*;
pub mod constants;
pub mod error;
pub mod event;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use event::*;
pub use instructions::*;
pub use state::state::*;
declare_id!("ALH6gghhaNkLbYZ9NadrYQUbqB3XSKnKo6ALgZE3htW");

#[program]
pub mod challenge_me {
    use super::*;

    pub fn initialize(ctx: Context<InitializeAccount>) -> Result<()> {
        _create_user(ctx)
    }
    pub fn upload_post(
        ctx: Context<UploadPostTask>,
        challenge_id: u64,
        post_id: u64,
        task_name: String,
        task_description: String,
        task_emoji: String,
        current_date: String,
        day: u64,
    ) -> Result<()> {
        _create_task(
            ctx,
            challenge_id,
            post_id,
            task_name,
            task_description,
            task_emoji,
            current_date,
            day,
        )
    }

    pub fn start_challenge(
        ctx: Context<ChallengeContext>,
        challenge_id: u64,
        challenge_type: ChallengeType,
    ) -> Result<()> {
        _start_challenge(ctx, challenge_id, challenge_type)
    }
}
