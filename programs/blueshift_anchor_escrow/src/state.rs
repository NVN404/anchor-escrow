use anchor_lang::prelude::*;

#[derive(InitSpace)]
// to calculate rent 
#[account(discriminator=1)]

pub struct Escrow{
    pub seed :u64,
    pub mint_a :Pubkey,
    pub mint_b: Pubkey,
    pub maker : Pubkey,
    pub receive:u64,
    pub bump : u8,
   
}