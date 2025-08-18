use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::Metadata, token::TokenAccount, token_interface::{Mint, TokenInterface}};

use crate::TokenLottery;

pub fn _buy_ticket(ctx: Context<BuyTicketContext>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct BuyTicketContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds=[b"token_lottery".as_ref()],
        bump = token_lottery.bump
    )]
    pub token_lottery: Account<'info, TokenLottery>,

    #[account(
        init,
        payer=payer,
        seeds=[token_lottery.total_tickets.to_le_bytes().as_ref()],
        bump,
        mint::decimals=0,
        mint::authority=collection_mint,
        mint::freeze_authority=collection_mint,
        mint::token_program= token_program
    )]
    pub ticket_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = ticket_mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program,
    )]
    pub destination: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut, 
        seeds=[
            b"metadata",
            token_metadata_program.key().as_ref(),
            ticket_mint.key().as_ref()
        ],
        bump,
        seeds::program = token_metadata_program.key()
    )]

    /// CHECK: this account 
    pub ticket_metadata : UncheckedAccount<'info>,


       /// CHECK : This accounct check by metadata smart contract
       #[account(
        mut, 
        seeds=[
            b"metadata",
            token_metadata_program.key().as_ref(),
            ticket_mint.key().as_ref(),
            b"edition"
        ],
        bump,  
        seeds::program = token_metadata_program.key()
    )]

    pub ticket_master_edition : UncheckedAccount<'info>,
    #[account(
        mut,
        seeds=[b"collection_mint".as_ref()],
        bump
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program : Program<'info,Metadata>,
    pub token_program: Interface<'info, TokenInterface>,
}
