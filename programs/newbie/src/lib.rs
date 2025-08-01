use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

// NOTE: You MUST use this exact program ID for the challenge
declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        // Check if vault is empty
        require_eq!(ctx.accounts.vault.lamports(), 0, VaultError::VaultAlreadyExists);
        
        // Ensure amount exceeds rent-exempt minimum
        let rent = Rent::get()?;
        require_gt!(amount, rent.minimum_balance(0), VaultError::InvalidAmount);
        
        // Transfer lamports from signer to vault
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.signer.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                }
            ), 
            amount
        )?;
        
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        // Check if vault has any lamports
        let vault_balance = ctx.accounts.vault.lamports();
        require_gt!(vault_balance, 0, VaultError::VaultEmpty);
        
        // Create PDA signer seeds
        let signer_seeds = &[
            b"vault",
            ctx.accounts.signer.key.as_ref(),
            &[ctx.bumps.vault]
        ];
        
        // Transfer all lamports from vault to signer
        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.signer.to_account_info(),
                },
                &[signer_seeds]
            ),
            vault_balance
        )?;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum VaultError {
    #[msg("Vault already exists")]
    VaultAlreadyExists,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Vault is empty")]
    VaultEmpty,
}