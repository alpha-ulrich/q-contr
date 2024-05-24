use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use crate::solana_program::entrypoint::ProgramResult;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("Cwpbe9jh3Ug5uBRvpoPkcHAhAQREjRysKBnXYZ6mV5Cr");

#[program]
mod usdt_sol_swap {
    use super::*;

    pub fn swap_usdt_for_sol(ctx: Context<Swap>, amount: u64) -> ProgramResult {
        // Transfer USDT from user to the program
        token::transfer(
            ctx.accounts.into_transfer_context(),
            amount,
        )?;
        
        // Simulate swapping USDT for SOL by transferring SOL from the program to the user
        // This is just a simulation; in a real scenario, you'd interact with Raydium or Serum
        **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.program_authority.to_account_info().try_borrow_mut_lamports()? += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_usdt_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub program_usdt_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub program_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Swap<'info> {
    fn into_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_usdt_account.to_account_info(),
                to: self.program_usdt_account.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }
}
