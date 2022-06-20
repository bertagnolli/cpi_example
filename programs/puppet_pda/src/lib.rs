use anchor_lang::prelude::*;

declare_id!("9NnwZErVieXf2sSzwGxDoDpjJ4JuwmXoFM71iENFabeM");

#[program]
mod puppet_pda {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let puppet = &mut ctx.accounts.puppet;
        //puppet.bump = puppet_acc_bump;
        puppet.authority = *ctx.accounts.authority.key;
        //puppet.count = 0; Initialising count using #[derive(Default)] instead
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>, data: u64) -> Result<()> {
        let puppet = &mut ctx.accounts.puppet;
        puppet.count = puppet.count.checked_add(data).unwrap();
        Ok(())
    }

    pub fn reset(ctx: Context<Reset>) -> Result<()> {
        let puppet = &mut ctx.accounts.puppet;
        puppet.count = 0;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(puppet_acc_bump: u8)]
pub struct Initialize<'info> {
    #[account(init, seeds = [b"puppet_account".as_ref()], bump, payer = payer, space = 40 + 8)]
    pub puppet: Account<'info, Data>,
    /// CHECK:
    #[account()]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, seeds = [b"puppet_account".as_ref()], bump, has_one = authority)]
    pub puppet: Account<'info, Data>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Reset<'info> {
    #[account(mut, seeds = [b"puppet_account".as_ref()], bump, has_one = authority)]
    pub puppet: Account<'info, Data>,
    pub authority: Signer<'info>,
}

#[account]
#[derive(Default)]
pub struct Data {
    pub authority: Pubkey, // 32 bytes
    pub count: u64, // 8 bytes
    //pub bump: u8,
}