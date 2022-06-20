use anchor_lang::prelude::*;
use puppet_pda::cpi::accounts::Increment;
use puppet_pda::cpi::accounts::Reset;
use puppet_pda::program::PuppetPda;
use puppet_pda::{self, Data};

declare_id!("Hurs2qqfR7hrtcBhjwVCzopDv9usjAZntjZ5ancUk9zm");

#[program]
mod puppet_master {
    use super::*;

    //Function solely used to create an account and store the bump for PDA
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let puppet_master = &mut ctx.accounts.puppet_master;
        puppet_master.bump = *ctx.bumps.get("pda_authority").unwrap();
        Ok(())
    }

    pub fn remote_increment(ctx: Context<RemoteIncrement>, add_value: u64) -> Result<()> {
        let cpi_program = ctx.accounts.puppet_program.to_account_info();
        let seeds = &[
            b"puppet_account".as_ref(),
            &[ctx.accounts.puppet_master.bump],
        ];
        let signer = &[&seeds[..]];
        let cpi_accounts = Increment {
            puppet: ctx.accounts.puppet.to_account_info(),
            authority: ctx.accounts.pda_authority.to_account_info() //ctx.accounts.puppet.authority
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        puppet_pda::cpi::increment(cpi_ctx, add_value)
    }

    pub fn remote_reset(ctx: Context<RemoteReset>) -> Result<()> {
        let cpi_program = ctx.accounts.puppet_program.to_account_info();
        let seeds = &[
            b"puppet_account".as_ref(),
            &[ctx.accounts.puppet_master.bump],
        ];
        let signer = &[&seeds[..]];
        let cpi_accounts = Reset {
            puppet: ctx.accounts.puppet.to_account_info(),
            authority: ctx.accounts.puppet.to_account_info() //ctx.accounts.puppet.authority
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        puppet_pda::cpi::reset(cpi_ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = 8 + 8)]
    pub puppet_master: Account<'info, BumpData>,
    #[account(seeds = [b"puppet_account".as_ref()], bump)]
    pub pda_authority: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RemoteIncrement<'info> {
    #[account(mut)]
    pub puppet: Account<'info, Data>, // Puppet account address (created by user)
    pub puppet_program: Program<'info, PuppetPda>, // Actual program address
    #[account(seeds = [b"puppet_account".as_ref()], bump)]
    pub pda_authority: AccountInfo<'info>,
    #[account()]
    pub puppet_master: Account<'info, BumpData>,
    // Even though the puppet program already checks that authority is a signer
    // using the Signer type here is still required because the anchor ts client
    // can not infer signers from programs called via CPIs
    //pub authority: Signer<'info>
}

#[derive(Accounts)]
pub struct RemoteReset<'info> {
    #[account(mut)]
    pub puppet: Account<'info, Data>, // Puppet account address (created by user)
    pub puppet_program: Program<'info, PuppetPda>, // Actual program address
    #[account(seeds = [b"puppet_account".as_ref()], bump)]
    pub pda_authority: AccountInfo<'info>,
    #[account()]
    pub puppet_master: Account<'info, BumpData>,
    // Even though the puppet program already checks that authority is a signer
    // using the Signer type here is still required because the anchor ts client
    // can not infer signers from programs called via CPIs
    //pub authority: Signer<'info>
}

#[account]
#[derive(Default)]
pub struct BumpData {
    pub bump: u8,
}