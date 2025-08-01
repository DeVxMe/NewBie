use anchor_lang::prelude::*;

declare_id!("EjhHyXLY5GoejHouWxuYWugXQkMyMf1vUEdrWsBNjXEh");

#[program]
pub mod newbie {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
