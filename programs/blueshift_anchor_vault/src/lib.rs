use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<VaultAction>) -> Result<()> {
        ctx.accounts.withdraw(&ctx.bumps)
    }
}

#[derive(Accounts)]
pub struct VaultAction <'info> {

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>

}

#[error_code]
pub enum VaultError {
    #[msg("Vault already exists")]
    VaultAlreadyExists,
    #[msg("Invalid amount")]
    InvalidAmount,
}


impl <'info> VaultAction <'info> {
    pub fn deposit (&mut self, amount: u64) -> Result<()> {
        // Check if vault is empty
        require_eq!(self.vault.lamports(), 0, VaultError::VaultAlreadyExists);

        // Ensure amount exceeds rent-exempt minimum
        require_gt!(amount, Rent::get()?.minimum_balance(0), VaultError::InvalidAmount);

        // let cpi_program = self.system_program.to_account_info();
        // let cpi_accounts = Transfer {
        //     from: self.signer.to_account_info(),
        //     to: self.vault.to_account_info()
        // };
        // let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // transfer(cpi_ctx, amount)?;

        transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.signer.to_account_info(),
                    to: self.vault.to_account_info()
                }
            ), amount)?;

        Ok(())
    }

    pub fn withdraw (&mut self, bumps: &VaultActionBumps ) -> Result<()> {
        require_neq!(self.vault.lamports(), 0, VaultError::InvalidAmount);

        let signer_seeds = &[
            b"vault",
            self.signer.key.as_ref(),
            &[bumps.vault]
        ];

        // let cpi_program = self.system_program.to_account_info();
        // let cpi_accounts = Transfer {
        //     from: self.vault.to_account_info(),
        //     to: self.signer.to_account_info()
        // };

        // let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &[&signer_seeds[..]]);

        // transfer(cpi_ctx, self.vault.lamports())?;

        transfer(CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            Transfer { 
                from: self.vault.to_account_info(), 
                to: self.signer.to_account_info()
            },
            &[&signer_seeds[..]]
            ), self.vault.lamports())?;

        Ok(())
    }

}