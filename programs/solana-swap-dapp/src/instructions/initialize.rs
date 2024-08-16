use anchor_lang::prelude::*;
use crate::{state::Controller};
use anchor_spl::token::{ Mint, TokenAccount};

use crate::{CONTROLLER_PDA_SEED,ESCROW_PDA_SEED, MAX_STRING_LEN};
use crate::errors::ControllerError;
use std::str;

/// Initialize functions will init a controller to control the swapper, and also create an escrow to hold MOVE token under the controller's authority

