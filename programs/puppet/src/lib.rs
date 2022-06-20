// use anchor_lang::prelude::*;

// declare_id!("9NnwZErVieXf2sSzwGxDoDpjJ4JuwmXoFM71iENFabeM");

// #[program]
// mod puppet {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         let puppet = &mut ctx.accounts.puppet;
//         puppet.authority = *ctx.accounts.authority.key;
//         puppet.count = 0;
//         Ok(())
//     }

//     pub fn increment(ctx: Context<Increment>, data: u64) -> Result<()> {
//         let puppet = &mut ctx.accounts.puppet;
//         puppet.count = puppet.count.checked_add(data).unwrap();
//         Ok(())
//     }

//     pub fn reset(ctx: Context<Reset>) -> Result<()> {
//         let puppet = &mut ctx.accounts.puppet;
//         puppet.count = 0;
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize<'info> {
//     #[account(init, payer = payer, space = 48)]
//     pub puppet: Account<'info, Data>,
//     /// CHECK:
//     #[account()]
//     pub authority: AccountInfo<'info>,
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct Increment<'info> {
//     #[account(mut, has_one = authority)]
//     pub puppet: Account<'info, Data>,
//     pub authority: Signer<'info>,
// }

// #[derive(Accounts)]
// pub struct Reset<'info> {
//     #[account(mut, has_one = authority)]
//     pub puppet: Account<'info, Data>,
//     pub authority: Signer<'info>,
// }

// #[account]
// pub struct Data {
//     pub authority: Pubkey,
//     pub count: u64,
// }