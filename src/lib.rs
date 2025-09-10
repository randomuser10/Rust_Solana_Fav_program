use anchor_lang::prelude::*;

declare_id!('11111');

#[program]
pub mod favorites{
    use super::*;
    pub fn set_favorites(ctx:Context<SetFavorites>) -> Result<()>{
        msg!("Greetings from {}", ctx.program_id);

        let user_key = ctx.accounts.user.key();

        msg!("User {user_key}'s favorite number is {number} 
              and favorite color is {color}
              and hobbies are {:?}", hobbies);
        
        ctx.accounts.favorites.set_inner(Favorites{
            number,
            color,
            hobbies,
    });
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favorites{
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5,50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavorites<'info>{
    #[account(mut)]

    pub user: Signer<'info>,

    #[account{
        init,
        payer = user,
        space = 8 + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    }]
    pub favorites = Account<'info, Favorites>,

    pub system_program = Program<'info, System>,

}