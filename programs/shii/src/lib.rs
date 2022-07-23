use anchor_lang::prelude::*;

declare_id!("AGWGdZc99A4hqSa8rjWsAEiJqKokapVifMJW9eCEmm9X");

#[program]
pub mod shii {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
