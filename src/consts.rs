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

pub const TEST_CONTENT: &str = r#"import {
  assertAccountExists,
  createKeyPairSignerFromBytes,
  type Address,
  type KeyPairSigner,
  type TransactionSigner,
} from "@solana/kit";
import { describe, it, expect, beforeAll } from "bun:test";
import {
  BUNCHOR_TEMPLATE_PROGRAM_ADDRESS,
  COUNTER_DISCRIMINATOR,
  getCounterDecoder,
  getDecrementInstruction,
  getIncrementInstruction,
  getInitializeInstruction,
} from "@client/index";
import { connect, getPDAAndBump, type Connection } from "solana-kite";

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

describe("counter", async () => {
  let authority: KeyPairSigner | TransactionSigner;
  let counterPda: Address;

  let connection: Connection;

  let getCounter: () => Promise<bigint>;
  const CLUSTER = process.env.CLUSTER || "localnet";

  beforeAll(async () => {
    // Bun is so fast, the websocket connection may not be ready yet
    await sleep(2000);
    connection = connect(CLUSTER);

    authority =
      process.env.KEYPAIR_BYTES && CLUSTER === "devnet"
        ? await createKeyPairSignerFromBytes(
            new Uint8Array(JSON.parse(process.env.KEYPAIR_BYTES)),
          )
        : await connection.createWallet();

    const counterPDAAndBump = await getPDAAndBump(
      BUNCHOR_TEMPLATE_PROGRAM_ADDRESS,
      [Buffer.from("counter")],
    );
    counterPda = counterPDAAndBump.pda;

    const getCounters = connection.getAccountsFactory(
      BUNCHOR_TEMPLATE_PROGRAM_ADDRESS,
      COUNTER_DISCRIMINATOR,
      getCounterDecoder(),
    );

    getCounter = async () => {
      const counters = await getCounters();
      expect(counters.length).toBe(1);

      const counter = counters[0]!;
      expect(counter.exists).toBe(true);

      assertAccountExists(counter);
      return counter.data.value;
    };
  });

  it("should inititalize", async () => {
    const initIx = getInitializeInstruction({
      user: authority,
      counter: counterPda,
    });

    const result = await connection.sendTransactionFromInstructions({
      feePayer: authority,
      instructions: [initIx],
      commitment: "confirmed",
    });

    expect(result).toBeTruthy();
  });

  it("should increment", async () => {
    const incrementIx = getIncrementInstruction({
      user: authority,
      counter: counterPda,
    });

    const result = await connection.sendTransactionFromInstructions({
      feePayer: authority,
      instructions: [incrementIx],
      commitment: "confirmed",
    });

    expect(result).toBeTruthy();

    const counter = await getCounter();
    expect(counter).toBe(1n);
  });

  it("should decrement", async () => {
    const decrementIx = getDecrementInstruction({
      user: authority,
      counter: counterPda,
    });

    const result = await connection.sendTransactionFromInstructions({
      feePayer: authority,
      instructions: [decrementIx],
      commitment: "confirmed",
    });

    expect(result).toBeTruthy();

    const counter = await getCounter();
    expect(counter).toBe(0n);
  });
});
"#;

pub fn codama_contents(project_name: &str) -> String {
    format!(r#"{{
  "idl": "./target/idl/{}.json",
  "scripts": {{
    "js": [
      {{
        "from": "@codama/renderers-js",
        "args": ["./app/generated/client"]
      }}
    ]
  }}
}}"#, project_name)
}