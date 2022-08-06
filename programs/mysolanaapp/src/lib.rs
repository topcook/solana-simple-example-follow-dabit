use anchor_lang::prelude::*;

declare_id!("AANeJEUnbhLNfHnSs24DJZ2qpqHbUYcZjtxgDdTnu9tE");

#[program]
pub mod mysolanaapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
