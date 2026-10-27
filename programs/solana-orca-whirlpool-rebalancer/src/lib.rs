use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod program_module {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Protocol initialized successfully");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
