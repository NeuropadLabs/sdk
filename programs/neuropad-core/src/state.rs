use anchor_lang::prelude::*;

#[account]
pub struct AgentAccount {
    pub owner: Pubkey,
    pub name: String,
    pub created_at: i64,
}
