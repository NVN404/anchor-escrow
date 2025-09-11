//accounts needed for make we are putting inside a struct that is only we passing in context

#[derive(accounts)]
#[instructions(seed:u64)]
pub struct make<'a>{
    #[account(mut)]
    pub maker: Signer<'a>,
    #[account(
        mut , 
        payer = maker , 
        space = Escrow::INIT_SPACE + Escrow::DISCRIMINATOR.len(),
        seeds = [b"escrow",maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump, 
    )]
    pub escrow:Account<'a,Escrow>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint_a : InterfaceAccount<'a,Mint>
    #[account(
        mint::token_program = token_program
    )]
    pub mint_b: InterfaceAccount<'a,Mint>
    #[account(
        mut , 
        associated_token::mint = mint_a,
        associated_token::authority = payer ,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_a : InterfaceAccount<'a,TokenAccount>
    #[account(
        mut , 
        payer = maker;
        associated_token::mint = mint_a,
        associated_token::authority = escrow ,
        associated_token::token_program=token_program,
    )]
    pub vault : InterfaceAccount<'a,TokenAccount>

    pub associated_token_program : program,'a , AssociatedToken>,
    pub system_program :Interface<'a,System>,
    pub token_program:Interface<'a,TokenInterface>,

}