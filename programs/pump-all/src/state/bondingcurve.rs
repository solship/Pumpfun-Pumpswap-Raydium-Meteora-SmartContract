use crate::errors::*;
use crate::events::CompleteEvent;
use crate::state::config::*;
use crate::utils::*;
use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[account]
pub struct BondingCurve {
    pub token_mint: Pubkey,
    pub creator: Pubkey,

    pub init_lamport: u64,

    pub token_total_supply: u64,

    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,

    pub real_sol_reserves: u64,
    pub real_token_reserves: u64,

    pub is_completed: bool,
}

#[derive(Debug, Clone)]
pub struct SellResult {
    pub token_amount: u64,
    pub sol_amount: u64,
}

#[derive(Debug, Clone)]
pub struct BuyResult {
    pub token_amount: u64,
    pub sol_amount: u64,
}

pub trait BondingCurveAccount<'info> {
    fn swap(
        &mut self,
        global_config: &Account<'info, Config>,
        token_mint: &Account<'info, Mint>,
        global_ata: &mut AccountInfo<'info>,
        user_ata: &mut AccountInfo<'info>,
        source: &mut AccountInfo<'info>,
        team_wallet: &mut AccountInfo<'info>,
        amount: u64,
        direction: u8,
        minimum_receive_amount: u64,

        user: &Signer<'info>,
        signer: &[&[&[u8]]],

        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<u64>;

    fn apply_sell(&mut self, token_amount: u64) -> Option<SellResult>;

    fn apply_buy(&mut self, sol_amount: u64) -> Option<BuyResult>;

    fn get_sol_for_sell_tokens(&self, token_amount: u64) -> Option<u64>;

    fn get_tokens_for_buy_sol(&self, sol_amount: u64) -> Option<u64>;
}

impl<'info> BondingCurveAccount<'info> for Account<'info, BondingCurve> {
    // solana pumpfun(pump.fun, pump fun) smart contract | forked and customized pumpfun(pump.fun, pump fun) smart contract, pumpfun(pump.fun, pump fun) + raydium cpi for pumpfun(pump.fun, pump fun) fork, pumpfun(pump.fun, pump fun) + meteora cpi for pumpfun(pump.fun, pump fun) fork, token 2022
    // **Telegram**: [@Tru3B1iss](https://t.me/Tru3B1iss)
    // **X (Twitter)**: [@XTruebliss](https://x.com/XTruebliss)
    // **Discord**: [@trueb1iss](https://discord.com/users/1274339638668038187)
}
