    use anchor_lang::prelude::*;

    declare_id!("DssdKJUV4NTz917Kii1JBbkk2wNFp828qEiqi7uj2v6o");

    #[program]
    pub mod anchor {
        use super::*;

        pub fn initialize_config(_ctx: Context<Initialize>) -> Result<()> {
            // Initialization logic goes here
            msg!("Configuration initialized successfully.");
            Ok(())
        }
    }

    #[derive(Accounts)]
    pub struct Initialize {

        #[account(mut)]
        pub payer: Signer<'info>,

        #[account(
            init,
            payer=payer,
            space=8 + TokenLottery::INIT_SPACE,
            seeds=[b"token_lottery".as_ref()],
            bump,
        )]
        pub token_lottery: Account<'info, TokenLottery>,

        pub system_program: Program<'info, System>,
    }

    #[account]
    #[derive(InitSpace)]
    pub struct TokenLottery {
        pub bump: u8,
        pub winner: u64
        pub winner_claimed: bool,
        pub start_time: u64,
        pub end_time: u64,
        pub lottery_pot_amount: u64,
        pub total_tickets: u64,
        pub ticket_price: u64,
        pub authority: Pubkey,
        pub randomness_account: Pubkey,
    }
