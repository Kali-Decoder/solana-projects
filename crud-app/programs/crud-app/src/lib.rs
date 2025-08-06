use anchor_lang::prelude::*;

declare_id!("51YrpYcgEHurgrqTi3EBTYUsuE4emTo4RfU8zYxKXvwa");

#[program]
pub mod crud_app {
    use super::*;

    pub fn create_operation(
        ctx: Context<CreateOperation>,
        operation_name: String,
        operation_description: String,
    ) -> Result<()> {

        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.operation_account.operation_name = operation_name;
        ctx.accounts.operation_account.operation_description = operation_description;
        ctx.accounts.operation_account.operation_account = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn update_operation(
        ctx: Context<UpdateOperation>,
        operation_name: String,
        operation_description: String,
    ) -> Result<()> {
        msg!("Journal Entry Updated");
        msg!("Title: {}", operation_name);
        msg!("Message: {}", operation_description);
        let operation_entry = &mut ctx.accounts.operation_account;
        operation_entry.operation_name = operation_name;
        operation_entry.operation_description = operation_description;
        Ok(())
    }

    pub fn delete_operation(ctx: Context<DeleteOperation>,operation_name:String) -> Result<()> {
        msg!("Deleting operation: {}", operation_name);
        let operation_account = &mut ctx.accounts.operation_account;
    
        msg!("Operation deleted successfully");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(operation_name: String, operation_description: String)]
pub struct CreateOperation<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = OperationAccount::INIT_SPACE,
        seeds = [operation_name.as_bytes(), authority.key().as_ref()],
        bump
    )]
    pub operation_account: Account<'info, OperationAccount>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(operation_name: String, operation_description: String)]

pub struct UpdateOperation<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [operation_name.as_bytes(), authority.key().as_ref()], 
        bump, 
        realloc = 8 + 32 + 4 + operation_name.len() + 4 + operation_description.len(),
        realloc::payer = authority, 
        realloc::zero = true, 
    )]
    pub operation_account: Account<'info, OperationAccount>,
    pub system_program: Program<'info, System>

}

#[derive(Accounts)]
#[instruction(operation_name: String)]
pub struct DeleteOperation<'info> {
    #[account( 
        mut, 
        seeds = [operation_name.as_bytes(), owner.key().as_ref()], 
        bump, 
        close= owner,
    )]
    pub operation_account: Account<'info, OperationAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct OperationAccount {
    pub operation_account: Pubkey,
    #[max_len(100)]
    pub operation_name: String,
    #[max_len(100)]
    pub operation_description: String,
}
