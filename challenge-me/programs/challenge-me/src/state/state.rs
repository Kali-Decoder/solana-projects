

use anchor_lang::prelude::*;

#[account]
pub struct UserProfile {
    pub owner : Pubkey,
    pub challenges: Vec<Pubkey>, 
}

#[account]
#[derive(InitSpace)]
pub struct Task{
    pub post_id : u64,
    pub owner : Pubkey,
    #[max_len(200)]
    pub title : String,
    #[max_len(1000)]
    pub discription:String,
    #[max_len(100)]
    pub emoji : String ,
    #[max_len(100)]
    pub current_time: String ,
    pub challenge : Pubkey,
    pub day : u64   
}

#[account]
pub struct Challenge {
    pub challenge_id : u64,
    pub owner: Pubkey,
    pub current_day: u32,
    pub total_days: u32,
    pub completed: bool,
    pub challenge_type: ChallengeType,
    pub posts: Vec<Pubkey>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum ChallengeType {
    OneWeek,
    OneMonth,
    TwoMonths,
    SixMonths,
    OneYear,
    SeventyFiveHard,
}

