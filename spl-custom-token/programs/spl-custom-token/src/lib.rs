use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, FreezeAccount, ThawAccount, MintTo, Approve};
declare_id!("SkCS47aCgRpg2QoVNnKB9xdvPPbuYUofLW4aoKSbyW8");

#[program]
pub mod spl_custom_token {
    use super::*;
    // Instruction one to initialize the program
    pub fn create_token_mint(ctx:Context<CreateTokenMint>,decimals:u8,mint_authority:Pubkey) -> Result<()> {
      
        Ok(())
    }

    // Instruction two for delegation 

    pub fn delegation_tokens(ctx:Context<DelegationTokens>,amount:u64) -> Result<()> {
          let cpi_accounts = Approve {
            to : ctx.accounts.token_account.to_account_info(),
            delegate : ctx.accounts.delegate.to_account_info(),
            owner : ctx.accounts.owner.to_account_info(),
        }
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        token::approve(cpi_context, amount)?;
    
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(decimals: u8, mint_authority: Pubkey)]
pub struct CreateTokenMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer=payer,
        mint::decimals = decimals,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
    )]
    pub mint : Account<'info,Mint>
    #[account(
        seeds = [b"authority"],
        bump
    )]
    pub program_authority : UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent : Sysvar<'info, Rent>,
}


#[derive(Accounts)]
pub struct DelegationTokens<'info> {
    #[account(mut)]
    pub token_account : Account<'info, TokenAccount>,

    /// CHECK : the account being delegate to
    pub delegate : UncheckedAccount<'info>,
    pub owner : Signer<'info>,
    pub token_program: Program<'info, Token>,
}