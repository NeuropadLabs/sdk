use anchor_lang::prelude::*;
mod state;
mod instructions;

use instructions::*;

declare_id!("NeuroPad11111111111111111111111111111111");

#[program]
pub mod neuropad_core {
    use super::*;

    pub fn register_agent(ctx: Context<RegisterAgent>, name: String) -> Result<()> {
        register::handler(ctx, name)
    }
}
