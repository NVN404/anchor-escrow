use anchor_lang::prelude::*;

#[derive(InitSpace)]
// to calculate rent 
#[account(discriminator=1)]

pub struct Escrow{
    pub seed :u64;
    pub mint_a :u64;
    pub mint_b: u64;
    pub maker : Pubkey;
    pub bump : u8;
    pub receive:Pubkey;
}