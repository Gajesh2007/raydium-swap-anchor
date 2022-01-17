//! Anchor-compatible SDK for the StableSwap program.
#![deny(missing_docs)]
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]
#![allow(clippy::nonstandard_macro_braces)]

use anchor_lang::solana_program::program_pack::Pack;
use anchor_lang::{prelude::*, solana_program};
use std::ops::Deref;
use std::option;
mod amm_instruction;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/// Creates an 'initialize' instruction.
pub fn initialize<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Initialize<'info>>,
    srm_token_account_pub: Option<Pubkey>,
    nonce: u8,
) -> ProgramResult {
    let ix = amm_instruction::initialize(
        ctx.accounts.program.key,
        ctx.accounts.amm_id.key,
        ctx.accounts.amm_authority.key,
        ctx.accounts.amm_open_orders.key,
        ctx.accounts.lp_mint_address.key,
        ctx.accounts.coin_mint_address.key,
        ctx.accounts.pc_mint_address.key,
        ctx.accounts.pool_coin_token_account.key,
        ctx.accounts.pool_pc_token_account.key,
        ctx.accounts.pool_withdraw_queue.key,
        ctx.accounts.pool_target_orders_account.key,
        ctx.accounts.pool_lp_token_account.key,
        ctx.accounts.pool_temp_lp_token_account.key,
        ctx.accounts.serum_program.key,
        ctx.accounts.serum_market.key,
        ctx.accounts.user_wallet.key,
        srm_token_account_pub,
        nonce,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.program,
            ctx.accounts.amm_id,
            ctx.accounts.amm_authority,
            ctx.accounts.amm_open_orders,
            ctx.accounts.lp_mint_address,
            ctx.accounts.coin_mint_address,
            ctx.accounts.pc_mint_address,
            ctx.accounts.pool_coin_token_account,
            ctx.accounts.pool_pc_token_account,
            ctx.accounts.pool_withdraw_queue,
            ctx.accounts.pool_target_orders_account,
            ctx.accounts.pool_lp_token_account,
            ctx.accounts.pool_temp_lp_token_account,
            ctx.accounts.serum_program,
            ctx.accounts.serum_market,
            ctx.accounts.user_wallet,
            ctx.accounts.srm_token_account,
        ],
        ctx.signer_seeds,
    )
}

/// Creates an 'deposit' instruction.
pub fn deposit<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Deposit<'info>>,
    max_coin_amount: u64,
    max_pc_amount: u64,
    base_side: u64,
) -> ProgramResult {
    let ix = amm_instruction::deposit(
        ctx.accounts.program.key,
        ctx.accounts.amm_id.key,
        ctx.accounts.amm_authority.key,
        ctx.accounts.amm_open_orders.key,
        ctx.accounts.amm_target_orders.key,
        ctx.accounts.lp_mint_address.key,
        ctx.accounts.pool_coin_token_account.key,
        ctx.accounts.pool_pc_token_account.key,
        ctx.accounts.serum_market.key,
        ctx.accounts.user_coin_token_account.key,
        ctx.accounts.user_pc_token_account.key,
        ctx.accounts.user_lp_token_account.key,
        ctx.accounts.user_owner.key,
        max_coin_amount,
        max_pc_amount,
        base_side,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.program,
            ctx.accounts.amm_id,
            ctx.accounts.amm_authority,
            ctx.accounts.amm_open_orders,
            ctx.accounts.amm_target_orders,
            ctx.accounts.lp_mint_address,
            ctx.accounts.pool_coin_token_account,
            ctx.accounts.pool_pc_token_account,
            ctx.accounts.serum_market,
            ctx.accounts.user_coin_token_account,
            ctx.accounts.user_pc_token_account,
            ctx.accounts.user_lp_token_account,
            ctx.accounts.user_owner,
        ],
        ctx.signer_seeds,
    )
}

// --------------------------------
// Instructions
// --------------------------------

/// Accounts for an [initialize] instruction.
#[derive(Accounts)]
pub struct Initialize<'info> {
    program: AccountInfo<'info>,
    amm_id: AccountInfo<'info>,
    amm_authority: AccountInfo<'info>,
    amm_open_orders: AccountInfo<'info>,
    lp_mint_address: AccountInfo<'info>,
    coin_mint_address: AccountInfo<'info>,
    pc_mint_address: AccountInfo<'info>,
    pool_coin_token_account: AccountInfo<'info>,
    pool_pc_token_account: AccountInfo<'info>,
    pool_withdraw_queue: AccountInfo<'info>,
    pool_target_orders_account: AccountInfo<'info>,
    pool_lp_token_account: AccountInfo<'info>,
    pool_temp_lp_token_account: AccountInfo<'info>,
    serum_program: AccountInfo<'info>,
    serum_market: AccountInfo<'info>,
    user_wallet: AccountInfo<'info>,
    srm_token_account: AccountInfo<'info>,
}

/// Accounts for an [deposit] instruction.
#[derive(Accounts)]
pub struct Deposit<'info> {
    program: AccountInfo<'info>,
    amm_id: AccountInfo<'info>,
    amm_authority: AccountInfo<'info>,
    amm_open_orders: AccountInfo<'info>,
    amm_target_orders: AccountInfo<'info>,
    lp_mint_address: AccountInfo<'info>,
    pool_coin_token_account: AccountInfo<'info>,
    pool_pc_token_account: AccountInfo<'info>,
    serum_market: AccountInfo<'info>,
    user_coin_token_account: AccountInfo<'info>,
    user_pc_token_account: AccountInfo<'info>,
    user_lp_token_account: AccountInfo<'info>,
    user_owner: AccountInfo<'info>,
}