pub fn counter_lib(project_name: &str) -> String {
    format!(
        r#"use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("7LBeQpPgzzjEWw4z7S5aJF3zyyqMFpKfMn1hSg7DKxL9");

#[program]
pub mod {} {{

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {{
        ctx.accounts.initialize(ctx.bumps.counter)
    }}

    pub fn increment(ctx: Context<Increment>) -> Result<()> {{
        ctx.accounts.increment()
    }}

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {{
        ctx.accounts.decrement()
    }}
}}
"#,
        project_name
    )
}

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
        space = 8 + Counter::INIT_SPACE,
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
        self.counter.value = self.counter
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
        self.counter.value = self.counter
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

pub fn package_json_contents(project_name: &str) -> String {
    format!(
        r#"{{
  "name": "{}",
  "scripts": {{
    "format:fix": "prettier tests/* --write",
    "format": "prettier tests/* --check",
    "generate": "codama run js",
    "setup": "anchor build && codama run js",
    "test": "anchor test",
    "test:devnet": "CLUSTER=DEVNET anchor test --provider devnet --skip-deploy"
  }},
  "dependencies": {{
    "@solana/kit": "^6.3.1",
    "solana-kite": "^3.2.1"
  }},
  "devDependencies": {{
    "@codama/nodes-from-anchor": "^1.3.9",
    "@codama/renderers-js": "^2.0.3",
    "@types/bun": "^1.3.10",
    "codama": "^1.5.1",
    "prettier": "^3.8.1"
  }}
}}"#,
        project_name
    )
}

pub const TS_CONFIG_CONTENTS: &str = r#"{
  "compilerOptions": {
    // Environment setup & latest features
    "lib": ["ESNext"],
    "target": "ESNext",
    "module": "Preserve",
    "moduleDetection": "force",
    "jsx": "react-jsx",
    "allowJs": true,

    // Bundler mode
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "verbatimModuleSyntax": true,
    "noEmit": true,

    // Best practices
    "strict": true,
    "skipLibCheck": true,
    "noFallthroughCasesInSwitch": true,
    "noUncheckedIndexedAccess": true,
    "noImplicitOverride": true,

    // Some stricter flags (disabled by default)
    "noUnusedLocals": false,
    "noUnusedParameters": false,
    "noPropertyAccessFromIndexSignature": false,
    
    "baseUrl": ".",
    "paths": {
      "@client/*": ["app/client/src/generated/*"]
    }
  }
}"#;

pub fn codama_contents(project_name: &str) -> String {
    format!(
        r#"{{
  "idl": "./target/idl/{}.json",
  "scripts": {{
    "js": [
      {{
        "from": "@codama/renderers-js",
        "args": ["./app/client"]
      }}
    ]
  }}
}}"#,
        project_name
    )
}

pub const BUNFIG_CONTENTS: &str = r#"[alias]
"@client/" = "./app/client/src/generated/"
"#;

pub fn anchor_contents(project_name: &str) -> String {
    format!(
        r#"[toolchain]
package_manager = "bun"

[features]
resolution = true
skip-lint = false

[programs.localnet]
{} = "7LBeQpPgzzjEWw4z7S5aJF3zyyqMFpKfMn1hSg7DKxL9"

[registry]
url = "https://api.apr.dev"

[provider]
cluster = "localnet"
wallet = "~/.config/solana/id.json"

[scripts]
test = "bun test --timeout 1000000 tests/*.test.ts"
"#,
        project_name
    )
}

// TODO!: This shi ugly
pub fn test_contents(const_name: &str) -> String {
    let first_half = r#"import {
  assertAccountExists,
  createKeyPairSignerFromBytes,
  type Address,
  type KeyPairSigner,
  type TransactionSigner,
} from "@solana/kit";
import { describe, it, expect, beforeAll } from "bun:test";
import { "#;
    let second_half = r#"
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
    format!(
        r#"{}
  {},{}"#,
        first_half, const_name, second_half
    )
}

pub const GITIGNORE_CONTENTS: &str = r#".anchor
.DS_Store
target
**/*.rs.bk
node_modules
test-ledger
.yarn
app/client
app/node_modules
app/dist"#;
