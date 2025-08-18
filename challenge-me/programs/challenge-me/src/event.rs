use anchor_lang::prelude::*;

#[event]
pub struct UserCreated {
    pub owner: Pubkey,
    pub bump: u8,
}

#[event]
pub struct TaskAdded {
    pub owner: Pubkey,
    pub task_id: u64,
}
