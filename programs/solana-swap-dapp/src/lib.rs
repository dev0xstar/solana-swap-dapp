use anchor_lang::prelude::*;

declare_id!("J1KNgjD2T5M1wzi9fqJhBCmcLV8yp1r86AnarpZvySST");

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

pub const CONTROLLER_PDA_SEED: &[u8] = b"controller";
pub const ESCROW_PDA_SEED: &[u8] = b"escrow";
pub const MAX_STRING_LEN: usize = 50;
#[program]
pub mod solana_swap_dapp {
    use super::*;




}

