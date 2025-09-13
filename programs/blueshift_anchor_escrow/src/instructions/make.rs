use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
         transfer_checked, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::{errors::EscrowError, state::Escrow};




//accounts needed for make we are putting inside a struct that is only we passing in context

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Make<'a>{
    #[account(mut)]
    pub maker: Signer<'a>,
    #[account(
        init , 
        payer = maker , 
        space = Escrow::INIT_SPACE + Escrow::DISCRIMINATOR.len(),
        seeds = [b"escrow",maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump, 
    )]
    pub escrow:Account<'a,Escrow>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint_a : InterfaceAccount<'a,Mint>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint_b: InterfaceAccount<'a,Mint>,
    #[account(
        mut , 
        associated_token::mint = mint_a,
        associated_token::authority = maker ,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_a : InterfaceAccount<'a,TokenAccount>,
    #[account(
        init, 
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow ,
        associated_token::token_program=token_program,
    )]
    pub vault : InterfaceAccount<'a,TokenAccount>,

    pub associated_token_program : Program<'a , AssociatedToken>,
    
    pub token_program:Interface<'a,TokenInterface>,
    pub system_program :Program<'a,System>,

}
impl<'a > Make<'a > {
    fn populate_escrow(&mut self, seed: u64, amount: u64, bump: u8) -> Result<()> {
        self.escrow.set_inner(Escrow {
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive: amount,
            bump,
        });
 
        Ok(())
    }
 
    fn deposit_tokens(&self, amount: u64) -> Result<()> {
        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.maker_ata_a.to_account_info(),
                    mint: self.mint_a.to_account_info(),
                    to: self.vault.to_account_info(),
                    authority: self.maker.to_account_info(),
                },
            ),
            amount,
            self.mint_a.decimals,
        )?;
 
        Ok(())
    }
}

pub fn handler(ctx: Context<Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
    require_gt!(receive, 0, EscrowError::InvalidAmount);
    require_gt!(amount, 0, EscrowError::InvalidAmount);
 
    ctx.accounts.populate_escrow(seed, receive, ctx.bumps.escrow)?;
 
    ctx.accounts.deposit_tokens(amount)?;
 
    Ok(())
}