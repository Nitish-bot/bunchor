pub const COUNTER_LIB: &str = r#"use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("7LBeQpPgzzjEWw4z7S5aJF3zyyqMFpKfMn1hSg7DKxL9");

#[program]
pub mod bunchor_template {

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
"#;

pub const COUNTER_ERRORS: &str = r#"use anchor_lang::prelude::*;

#[error_code]
pub enum TemplateError {
    #[msg("An integer underflowed")]
    IntegerUnderflow,
    #[msg("An integer overflowed")]
    IntegerOverflow,
}
"#;

pub const COUNTER_INSTRUCTIONS_MOD: &str = r#"pub mod decrement;
pub mod increment;
pub mod initialize;

pub use decrement::*;
pub use increment::*;
pub use initialize::*;
"#;

pub const COUNTER_INITIALIZE: &str = r#"use crate::state::Counter;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = Counter::INIT_SPACE,
        seeds = [b"counter"],
        bump,
    )]
    pub counter: Account<'info, Counter>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bump: u8) -> Result<()> {
        self.counter.bump = bump;
        Ok(())
    }
}
"#;

pub const COUNTER_INCREMENT: &str = r#"use crate::{errors::TemplateError, state::Counter};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Increment<'info> {
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"counter"],
        bump
    )]
    pub counter: Account<'info, Counter>,
}

impl<'info> Increment<'info> {
    pub fn increment(&mut self) -> Result<()> {
        self.counter
            .value
            .checked_add(1)
            .ok_or(TemplateError::IntegerOverflow)?;
        Ok(())
    }
}
"#;

pub const COUNTER_DECREMENT: &str = r#"use crate::{errors::TemplateError, state::Counter};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Decrement<'info> {
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"counter"],
        bump
    )]
    pub counter: Account<'info, Counter>,
}

impl<'info> Decrement<'info> {
    pub fn decrement(&mut self) -> Result<()> {
        self.counter
            .value
            .checked_sub(1)
            .ok_or(TemplateError::IntegerUnderflow)?;
        Ok(())
    }
}
"#;

pub const COUNTER_STATE_MOD: &str = r#"pub mod counter;

pub use counter::*;
"#;

pub const COUNTER_STATE_COUNTER: &str = r#"use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub value: u64,
    pub bump: u8,
}
"#;

pub const TEST_CONTENT: &str = r#"import { test, expect } from "bun:test";

test("sample test", () => {
    expect(1 + 1).toBe(2);
});
"#;
