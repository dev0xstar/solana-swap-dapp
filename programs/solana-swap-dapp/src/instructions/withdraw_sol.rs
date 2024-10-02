use anchor_lang::{prelude::*};
use crate::{state::Controller, CONTROLLER_PDA_SEED};


pub fn withdraw_sol(
    ctx: Context<WithdrawSol>,
    
)-> Result<()> {
    let controller = &mut ctx.accounts.controller;
    let initializer = &ctx.accounts.initializer;
    
    let withdraw_amount = controller.sol_received - controller.sol_claimed;
    controller.sol_claimed += withdraw_amount;

    **controller.to_account_info().try_borrow_mut_lamports()? -= withdraw_amount;
    **initializer.try_borrow_mut_lamports()? += withdraw_amount;

    Ok(())
}

