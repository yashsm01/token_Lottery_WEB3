use anchor_lang::prelude::*;

declare_id!("DssdKJUV4NTz917Kii1JBbkk2wNFp828qEiqi7uj2v6o");

#[program]
pub mod anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
