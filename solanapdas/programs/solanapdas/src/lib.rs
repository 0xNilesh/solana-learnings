use anchor_lang::prelude::*;

declare_id!("HRK3YVtfwiLDLoZBquuwj6StG33HXYUvz16hRSgSS94L");

#[program]
pub mod solanapdas {
    use anchor_lang::solana_program::{
        entrypoint::ProgramResult, program::invoke, system_instruction::transfer,
    };

    use super::*;

    pub fn create(ctx: Context<Create>, name: String) -> ProgramResult {
        let bank: &mut Account<'_, Bank> = &mut ctx.accounts.bank;
        bank.name = name;
        bank.balance = 0;
        bank.admin = *ctx.accounts.user.key;
        Ok({})
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> ProgramResult {
        let txn = transfer(&ctx.accounts.user.key(), &ctx.accounts.bank.key(), amount);
        let _ = invoke(
            &txn,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.bank.to_account_info(),
            ],
        );
        let bank: &mut Account<'_, Bank> = &mut ctx.accounts.bank;
        bank.balance += amount;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let bank: &mut Account<'_, Bank> = &mut ctx.accounts.bank;
        let user: &mut _ = &mut ctx.accounts.user;

        if bank.admin != user.key() {
            return Err(ProgramError::IncorrectProgramId);
        }
        let rent = Rent::get()?.minimum_balance(bank.to_account_info().data_len());
        if **bank.to_account_info().lamports.borrow() - rent < amount {
            return Err(ProgramError::InsufficientFunds);
        }

        **bank.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer=user, space=5000, seeds=[b"bank", user.key().as_ref()], bump)]
    pub bank: Account<'info, Bank>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub bank: Account<'info, Bank>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub bank: Account<'info, Bank>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Bank {
    name: String,
    balance: u64,
    admin: Pubkey,
}
