/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use std::collections::{HashMap, HashSet};
use std::time::SystemTime;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, AccountId};

// Define the default message
const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

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
            next_period_timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() + 600,
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
    pub fn add_player(&mut self, player_address: AccountId) {
        // verify that game still in buy-in period
        // transfer buy-in from player
        // add player to random color
        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let side = match timestamp.as_nanos() & 1 { // good enough for government work
            0 => &mut self.white_players,
            1 => &mut self.black_players,
            _ => unreachable!(),
        };
        side.insert(player_address);
        todo!();
    }

    // add vote to current period votes
    pub fn cast_vote(&mut self, player_address: AccountId, board_fen: String, vote_fen: String) {
        // verify player in color to move
        let players_to_move = match self.fen_state.split(" ").nth(1).expect("couldn't parse internal FEN string") {
            "w" => &self.white_players,
            "b" => &self.black_players,
            _ => return,
        };
        if !players_to_move.contains(&player_address) || self.voted_this_period.contains(&player_address) {
            return;
        }
        // verify voter has correct board state
        if board_fen != self.fen_state { return; }
        // add vote to votes
        *self.votes.entry(vote_fen).or_insert(0) += 1;
    }

    // verify next vote timestamp is in the past, select winnning vote and update state
    pub fn tally_votes(&mut self) {
        // find highest voted fen
        let mut curr_most_votes = 0;
        let mut winning_fen = &self.fen_state;
        for (key, val) in &self.votes {
            if *val > curr_most_votes {
                curr_most_votes = *val;
                winning_fen = key;
            }
        }
        // parse for end game states
        // empty votes map
        self.votes.clear();
        // empty voted_this_period map
        self.voted_this_period.clear();
        // set next vote timestamp

        todo!();
    }

    fn finish_game(&mut self, end_state: EndState) {
        // mark game as over
        // take 5% out of pot for dao
        // distribute remaining pot to winners
        todo!();
        self.game_active = false
    }

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
