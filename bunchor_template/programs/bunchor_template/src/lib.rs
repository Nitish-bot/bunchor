use anchor_lang::prelude::*;

declare_id!("7LBeQpPgzzjEWw4z7S5aJF3zyyqMFpKfMn1hSg7DKxL9");

#[program]
pub mod bunchor_template {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
