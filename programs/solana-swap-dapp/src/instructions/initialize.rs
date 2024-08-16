use anchor_lang::prelude::*;
use crate::{state::Controller};
use anchor_spl::token::{ Mint, TokenAccount};

use crate::{CONTROLLER_PDA_SEED,ESCROW_PDA_SEED, MAX_STRING_LEN};
use crate::errors::ControllerError;
use std::str;

/// Initialize functions will init a controller to control the swapper, and also create an escrow to hold MOVE token under the controller's authority
pub fn initialize(
    ctx: Context<Initialize>,
    id: String,
    token_price: Vec<u64>,
    token_decimal: u8
)-> Result<()> {
    require!(token_price.len() == 2, ControllerError::InvalidPrice);
    require!(id.chars().count()< MAX_STRING_LEN, ControllerError::InvalidID);

    let controller = &mut ctx.accounts.controller;
    let initializer = &ctx.accounts.initializer;
    let token_mint = &ctx.accounts.token_mint;

    controller.initializer = initializer.key();
    controller.id = id.clone();
    controller.token_mint = token_mint.key();
    controller.token_price = token_price;
    controller.token_decimal = token_decimal;
    controller.sol_received = 0;
    controller.sol_claimed = 0;
    controller.bump = *ctx.bumps.get(str::from_utf8(CONTROLLER_PDA_SEED).unwrap()).unwrap();
    controller.escrow_bump = *ctx.bumps.get(str::from_utf8(ESCROW_PDA_SEED).unwrap()).unwrap();
    
    msg!("action: initialize");
    msg!("initializer: {}", initializer.key());
    Ok(())
}

