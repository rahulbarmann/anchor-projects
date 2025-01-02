use anchor_lang::prelude::*;

declare_id!("2dqL5aKDJAkdo3JjC4mqyADRKLQrdp3fDqswS7hHnvm2");

const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favorites {
    use super::*;

    pub fn add_favorite(
        ctx: Context<AddFavorite>,
        number: u64,
        color: String,
        shows: Vec<String>,
    ) -> Result<()> {
        let signer_pub_key = ctx.accounts.signer.key();
        msg!("Hello brother {} !! Now your new favorite things are: number: {}, color: {}, and shows: {:?}", signer_pub_key,  number, color, shows);
        ctx.accounts.fav_account.set_inner(Favorites {
            number,
            color,
            shows,
        });
        Ok(())
    }

    pub fn get_favorite(ctx: Context<AddFavorite>) -> Result<(u64, String, Vec<String>)> {
        let favorites_things = &ctx.accounts.fav_account;
        msg!("Number: {}", favorites_things.number);
        msg!("Color: {}", favorites_things.color);
        msg!("Shows: {:?}", favorites_things.shows);
        Ok((
            favorites_things.number,
            favorites_things.color.clone(),
            favorites_things.shows.clone(),
        ))
    }

}

#[derive(Accounts)]
pub struct AddFavorite<'info> {
    #[account(
        init, 
        payer = signer, 
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", signer.key().as_ref()], bump
        )]
    pub fav_account: Account<'info, Favorites>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub shows: Vec<String>,
}
