/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, Duration};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, require, AccountId, Promise};

// Define the default message
const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const VOTING_PERIOD_INTERVAL: u64 = 30;

enum EndState {
    WHITEWIN,
    BLACKWIN,
    STALEMATE,
    DRAW,
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    game_active: bool,
    fen_state: String,
    buyin_amount: u128,
    next_period_timestamp: u64,
    votes: HashMap<String, u64>,
    voted_this_period: HashSet<AccountId>,
    white_players: HashSet<AccountId>,
    black_players: HashSet<AccountId>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self {
            game_active: true,
            fen_state: STARTING_POSITION.to_string(),
            buyin_amount: 1_000_000_000_000_000_000_000_000,  // 1 near in yoctonear
            next_period_timestamp: env::block_timestamp() + VOTING_PERIOD_INTERVAL,
            votes: HashMap::new(),
            voted_this_period: HashSet::new(),
            white_players: HashSet::new(),
            black_players: HashSet::new(),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    pub fn get_fen(self) -> String {
        self.fen_state
    }

    fn player_has_stnear(&self, player_address: &AccountId) -> bool { true }

    #[payable]
    pub fn add_player(&mut self, player_address: AccountId) {
        // verify that game still in buy-in period
        //require!(self.fen_state.split(" ").last() != Some("1"), "buy-in period is over");
        // transfer buy-in to contract
        //require!(env::attached_deposit() > self.buyin_amount, "send more coins lol");  // using payable function
        require!(self.player_has_stnear(&player_address), "account must have stNEAR to play!");
        // add player to random color
        if self.white_players.is_empty() {
            self.white_players.insert(player_address);
        } else if self.black_players.is_empty() {
            self.black_players.insert(player_address);
        } else {
            let side = match env::block_timestamp_ms() & 1 {  // good enough for government work
                0 => &mut self.white_players,
                1 => &mut self.black_players,
                _ => unreachable!(),
            };
            side.insert(player_address);
        }
    }

    // add vote to current period votes
    pub fn cast_vote(&mut self, board_fen: String, vote_fen: String) {
        let player_address = env::signer_account_id();
        // verify voter has correct board state
        require!(board_fen == self.fen_state, "out of date board state");
        // verify player has voting rights
        require!(self.white_players.contains(&player_address) || self.black_players.contains(&player_address), "player can't vote in this game");
        // verify player hasn't voted this period
        require!(!self.voted_this_period.contains(&player_address), "player already voted this period");
        // verify player in color to move
        let players_to_move = match self.fen_state.split(" ").nth(1) {
            Some("w") => &self.white_players,
            Some("b") => &self.black_players,
            _ => env::panic_str("malformed FEN"),
        };
        require!(players_to_move.contains(&player_address), "it's not your turn");
        // add vote to votes
        *self.votes.entry(vote_fen).or_insert(0) += 1;
    }

    // verify next vote timestamp is in the past, select winnning vote and update state
    pub fn tally_votes(&mut self) {
        // verify next vote timestamp is in the past
        require!(env::block_timestamp() > self.next_period_timestamp, "current voting period not over");
        // find highest voted fen
        let mut curr_most_votes = 0;
        let mut winning_fen = &self.fen_state;
        for (key, val) in &self.votes {
            if *val > curr_most_votes {
                curr_most_votes = *val;
                winning_fen = key;
            }
        }
        // // parse for end game states
        // match winning_fen.split(" ").last() {
        //     Some("RESIGN") => { todo!(); },
        //     Some("OFFER_DRAW") => { todo!(); },
        //     Some("other stuff") => { todo!(); },
        //     _ => (),
        // };
        self.fen_state = winning_fen.to_owned();
        // empty votes map
        self.votes.clear();
        // empty voted_this_period set
        self.voted_this_period.clear();
        // set next vote timestamp
        self.next_period_timestamp = env::block_timestamp() + VOTING_PERIOD_INTERVAL;

    }

    // fn finish_game(&mut self, end_state: EndState) {
    //     // mark game as over
    //     self.game_active = false;
    //     // take 5% out of pot for dao
    //     todo!();
    //     // distribute remaining pot to winners
    //     todo!();
    // }

    // // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    // pub fn get_greeting(&self) -> String {
    //     return self.message.clone();
    // }

    // // Public method - accepts a greeting, such as "howdy", and records it
    // pub fn set_greeting(&mut self, message: String) {
    //     // Use env::log to record logs permanently to the blockchain!
    //     log!("Saving greeting {}", message);
    //     self.message = message;
    // }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn get_default_greeting() {
    //     let contract = Contract::default();
    //     // this test did not call set_greeting so should return the default "Hello" greeting
    //     assert_eq!(
    //         contract.get_greeting(),
    //         "Hello".to_string()
    //     );
    // }

    // #[test]
    // fn set_then_get_greeting() {
    //     let mut contract = Contract::default();
    //     contract.set_greeting("howdy".to_string());
    //     assert_eq!(
    //         contract.get_greeting(),
    //         "howdy".to_string()
    //     );
    // }
}
