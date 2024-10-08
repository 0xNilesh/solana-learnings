use anchor_lang::prelude::*;

declare_id!("5cCVT4u7Nj95SmjykkMVPmHZn4mMTqQqKq2oSeTNKST3");

#[program]
pub mod calculator {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn create(ctx: Context<Create>, init_message: String) -> ProgramResult {
        let calculator: &mut Account<'_, Calculator> = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok({})
    }

    pub fn add(ctx: Context<Addition>, a: i64, b: i64) -> ProgramResult {
        let calculator: &mut Account<'_, Calculator> = &mut ctx.accounts.calculator;
        calculator.result = a + b;
        Ok({})
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer=user, space=264)]
    pub calculator: Account<'info, Calculator>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[account]
pub struct Calculator {
    pub greeting: String,
    pub result: i64,
}
