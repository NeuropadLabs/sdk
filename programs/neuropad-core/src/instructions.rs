use anchor_lang::prelude::*;
use crate::state::AgentAccount;

pub mod register {
    use super::*;

    pub fn handler(ctx: Context<RegisterAgent>, name: String) -> Result<()> {
        let agent = &mut ctx.accounts.agent;
        agent.owner = ctx.accounts.user.key();
        agent.name = name;
        agent.created_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterAgent<'info> {
    #[account(init, payer = user, space = 8 + 64)]
    pub agent: Account<'info, AgentAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
