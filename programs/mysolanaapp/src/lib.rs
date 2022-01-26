use anchor_lang::prelude::*;
use std::time::{Duration, SystemTime};

declare_id!("Cf61PJ1yiLmHLbmTKk9uRkVtQTssNU1PfjF7FQbBRnMQ");

#[program]
mod mysolanaapp {
    use super::*;

    pub fn breeding(ctx: Context<Breeding>, data: Vec<u32>) -> ProgramResult {
        let now = System::now();
        let base_account = &mut ctx.accounts.base_account;
        let copy = data.clone();
        base_account.data = data;
        base_account.data_list.push(copy);
        base_account.breededAt = now();
        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let copy = data.clone();
        base_account.data = data;
        base_account.data_list.push(copy);
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Breeding<'info> {
    #[account(init, payer = user, space = 64 + 64)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub data: String,
    pub data_list: Vec<String>,
}