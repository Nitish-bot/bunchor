use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("8ujTgw9VFAw1cPDh125eKBVYbJjeFfBDbxZ6Dj56ko6d");

#[program]
pub mod template {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(ctx.bumps.counter)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.increment()
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        ctx.accounts.decrement()
    }
}
