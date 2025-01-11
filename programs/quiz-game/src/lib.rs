use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("your_program_id_here");

#[program]
pub mod quiz_game {
    use super::*;

    pub fn initialize_game(
        ctx: Context<InitializeGame>,
        bet_amount: u64,
        room_id: String,
    ) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.creator = ctx.accounts.creator.key();
        game.bet_amount = bet_amount;
        game.room_id = room_id;
        game.player_count = 1;
        game.is_active = true;
        Ok(())
    }

    pub fn join_game(
        ctx: Context<JoinGame>,
        room_id: String,
    ) -> Result<()> {
        let game = &mut ctx.accounts.game;
        require!(game.is_active, ErrorCode::GameNotActive);
        require!(game.player_count < 2, ErrorCode::GameFull);

        // Transfer tokens from player to game vault
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.player_token_account.to_account_info(),
                to: ctx.accounts.game_vault.to_account_info(),
                authority: ctx.accounts.player.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, game.bet_amount)?;

        game.player_count += 1;
        Ok(())
    }

    pub fn end_game(
        ctx: Context<EndGame>,
        winner: Pubkey,
    ) -> Result<()> {
        let game = &mut ctx.accounts.game;
        require!(game.is_active, ErrorCode::GameNotActive);

        // Calculate winner's prize (90% of total pot, 10% platform fee)
        let total_pot = game.bet_amount * 2;
        let winner_prize = total_pot * 90 / 100;

        // Transfer tokens to winner
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.game_vault.to_account_info(),
                to: ctx.accounts.winner_token_account.to_account_info(),
                authority: ctx.accounts.game_vault.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, winner_prize)?;

        game.is_active = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        space = 8 + 32 + 8 + 32 + 1 + 1
    )]
    pub game: Account<'info, Game>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinGame<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub player_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub game_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct EndGame<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub game_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub winner_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Game {
    pub creator: Pubkey,
    pub bet_amount: u64,
    pub room_id: String,
    pub player_count: u8,
    pub is_active: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Game is not active")]
    GameNotActive,
    #[msg("Game is full")]
    GameFull,
} 