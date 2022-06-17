use anchor_lang::prelude::*;
use puppet::cpi::accounts::Increment;
use puppet::cpi::accounts::Reset;
use puppet::program::Puppet;
use puppet::{self, Data};

declare_id!("Hurs2qqfR7hrtcBhjwVCzopDv9usjAZntjZ5ancUk9zm");

#[program]
mod puppet_master {
    use super::*;

    pub fn remote_increment(ctx: Context<RemoteIncrement>, add_value: u64) -> Result<()> {
        let cpi_program = ctx.accounts.puppet_program.to_account_info();
        let cpi_accounts = Increment {
            puppet: ctx.accounts.puppet.to_account_info(),
            authority: ctx.accounts.puppet.to_account_info() //ctx.accounts.puppet.authority
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        puppet::cpi::increment(cpi_ctx, add_value)
    }

    pub fn remote_reset(ctx: Context<RemoteReset>) -> Result<()> {
        let cpi_program = ctx.accounts.puppet_program.to_account_info();
        let cpi_accounts = Reset {
            puppet: ctx.accounts.puppet.to_account_info(),
            authority: ctx.accounts.puppet.to_account_info() //ctx.accounts.puppet.authority
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        puppet::cpi::reset(cpi_ctx)
    }
}

#[derive(Accounts)]
pub struct RemoteIncrement<'info> {
    #[account(mut)]
    pub puppet: Account<'info, Data>, // Puppet account address (created by user)
    pub puppet_program: Program<'info, Puppet>, // Actual program address
    // Even though the puppet program already checks that authority is a signer
    // using the Signer type here is still required because the anchor ts client
    // can not infer signers from programs called via CPIs
    pub authority: Signer<'info>
}

#[derive(Accounts)]
pub struct RemoteReset<'info> {
    #[account(mut)]
    pub puppet: Account<'info, Data>, // Puppet account address (created by user)
    pub puppet_program: Program<'info, Puppet>, // Actual program address
    // Even though the puppet program already checks that authority is a signer
    // using the Signer type here is still required because the anchor ts client
    // can not infer signers from programs called via CPIs
    pub authority: Signer<'info>
}