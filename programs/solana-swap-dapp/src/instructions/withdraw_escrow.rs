use anchor_lang::{prelude::*};
use crate::{state::Controller, CONTROLLER_PDA_SEED, ESCROW_PDA_SEED};
use anchor_spl::token::{TokenAccount, Transfer};


//use crate::errors::WithdrawSolError;

pub fn withdraw_escrow(
    ctx: Context<WithdrawEscrow>,
    
)-> Result<()> {
    let controller = &mut ctx.accounts.controller;
    let escrow = &ctx.accounts.escrow;
    let initializer_token_acconunt = &ctx.accounts.initializer_token_account;
    let token_program = &ctx.accounts.token_program;

    //********** Transfer token amount to initializer  ********* */
    let bump_vector = controller.bump.to_le_bytes();

    let inner = vec![

}