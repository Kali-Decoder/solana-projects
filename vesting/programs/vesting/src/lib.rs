use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        self as token_ix, Mint, TokenAccount, TokenInterface, TransferChecked,
    },
};

declare_id!("GgSjd1Gj6AHo43cQP2CFTTpw2Gps8NCWnmaagL9trjTg");

#[program]
pub mod vesting {
    use super::*;

    pub fn create_vesting_account(
        ctx: Context<CreateVestingAccount>,
        company_name: String,
    ) -> Result<()> {
        *ctx.accounts.vesting_account = VestingAccount {
            owner: ctx.accounts.signer.key(),
            mint: ctx.accounts.mint.key(),
            treasury_token_account: ctx.accounts.treasury_token_account.key(),
            company_name,
            treasury_bump: ctx.bumps.treasury_token_account,
            bump: ctx.bumps.vesting_account,
        };
        Ok(())
    }

    pub fn create_employee_account(
        ctx: Context<CreateEmployeeVesting>,
        start_time: i64,
        end_time: i64,
        total_amount: i64,
        cliff_time: i64,
    ) -> Result<()> {
        *ctx.accounts.employee_account = EmployeeAccount {
            beneficiary: ctx.accounts.beneficiary.key(),
            start_time,
            end_time,
            total_amount,
            total_withdrawn: 0,
            cliff_time,
            vesting_account: ctx.accounts.vesting_account.key(),
            bump: ctx.bumps.employee_account,
        };
        Ok(())
    }

    pub fn claim_tokens(ctx: Context<ClaimTokens>, company_name: String) -> Result<()> {
        let employee = &mut ctx.accounts.employee_account;
        let now = Clock::get()?.unix_timestamp;

        require!(now >= employee.cliff_time, ErrorCode::ClaimNotAvailableYet);

        let total_vesting_time = employee
            .end_time
            .checked_sub(employee.start_time)
            .ok_or(ErrorCode::MathOverflow)?;
        require!(total_vesting_time > 0, ErrorCode::MathOverflow);

        let vested_amount = if now >= employee.end_time {
            employee.total_amount
        } else {
            let elapsed = now.saturating_sub(employee.start_time).max(0);
            (employee.total_amount as i128 * elapsed as i128 / total_vesting_time as i128) as i64
        };

        let claimable_amount = vested_amount.saturating_sub(employee.total_withdrawn);
        require!(claimable_amount > 0, ErrorCode::NothingToClaim);

        // CPI: transfer_checked via Token-2022 interface
        let accounts = TransferChecked {
            from: ctx.accounts.treasury_token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.employee_token_account.to_account_info(),
            // ✅ authority is the vesting PDA (owner of treasury_token_account)
            authority: ctx.accounts.vesting_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();

        // ✅ signer seeds for the vesting PDA (matches CreateVestingAccount seeds)
        let signer_seeds: &[&[&[u8]]] = &[&[
            ctx.accounts.vesting_account.company_name.as_bytes(),
            &[ctx.accounts.vesting_account.bump],
        ]];

        let cpi_ctx = CpiContext::new(cpi_program, accounts).with_signer(signer_seeds);
        let decimals = ctx.accounts.mint.decimals;

        token_ix::transfer_checked(cpi_ctx, claimable_amount as u64, decimals)?;

        employee.total_withdrawn = employee
            .total_withdrawn
            .checked_add(claimable_amount)
            .ok_or(ErrorCode::MathOverflow)?;

        Ok(())
    }
}

/* ------------------------------- Accounts -------------------------------- */

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct CreateVestingAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + VestingAccount::INIT_SPACE,
        seeds = [company_name.as_bytes()],
        bump
    )]
    pub vesting_account: Account<'info, VestingAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    // PDA-owned token-2022 account that holds the treasury funds
    #[account(
        init,
        payer = signer,
        seeds = [b"vesting_treasury", company_name.as_bytes()],
        bump,
        token::mint = mint,
        // ✅ owner/authority of this token account is the vesting PDA
        token::authority = vesting_account,
        token::token_program = token_program
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[account]
#[derive(InitSpace)]
pub struct VestingAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub treasury_token_account: Pubkey,
    #[max_len(50)]
    pub company_name: String,
    pub treasury_bump: u8,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct CreateEmployeeVesting<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: system account is fine for beneficiary
    pub beneficiary: SystemAccount<'info>,

    #[account(mut, has_one = owner)]
    pub vesting_account: Account<'info, VestingAccount>,

    #[account(
        init,
        payer = owner,
        space = 8 + EmployeeAccount::INIT_SPACE,
        seeds = [b"employee_vesting", beneficiary.key().as_ref(), vesting_account.key().as_ref()],
        bump
    )]
    pub employee_account: Account<'info, EmployeeAccount>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct EmployeeAccount {
    pub beneficiary: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
    pub total_amount: i64,
    pub total_withdrawn: i64,
    pub cliff_time: i64,
    pub vesting_account: Pubkey,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct ClaimTokens<'info> {
    #[account(mut)]
    pub beneficiary: Signer<'info>,

    #[account(
        mut,
        has_one = beneficiary,
        seeds = [b"employee_vesting", beneficiary.key().as_ref(), vesting_account.key().as_ref()],
        bump = employee_account.bump
    )]
    pub employee_account: Account<'info, EmployeeAccount>,

    #[account(
        mut,
        seeds = [company_name.as_bytes()],
        bump = vesting_account.bump,
        has_one = treasury_token_account,
        has_one = mint
    )]
    pub vesting_account: Account<'info, VestingAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = beneficiary,
        associated_token::mint = mint,
        associated_token::authority = beneficiary,
        associated_token::token_program = token_program
    )]
    pub employee_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

/* -------------------------------- Errors --------------------------------- */

#[error_code]
pub enum ErrorCode {
    #[msg("Claiming is not available yet.")]
    ClaimNotAvailableYet,
    #[msg("There is nothing to claim.")]
    NothingToClaim,
    #[msg("Math overflow")]
    MathOverflow,
}
