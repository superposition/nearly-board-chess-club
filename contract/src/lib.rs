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

enum EndState {
    WHITEWIN,
    BLACKWIN,
    STALEMATE,
    DRAW,
}

// TODO: get timestamp from near_sdk?
fn unix_epoch_duration() -> Duration {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()
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
            next_period_timestamp: unix_epoch_duration().as_secs() + 600,
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

    #[payable]
    pub fn add_player(&mut self, player_address: AccountId) {
        // verify that game still in buy-in period
        require!(self.fen_state.split(" ").last() != Some("1"), "buy-in period is over");
        // transfer buy-in to contract
        require!(env::attached_deposit() > self.buyin_amount, "send more coins lol");  // using payable function
        // add player to random color
        let side = match unix_epoch_duration().as_nanos() & 1 {  // good enough for government work
            0 => &mut self.white_players,
            1 => &mut self.black_players,
            _ => unreachable!(),
        };
        side.insert(player_address);
    }

    // add vote to current period votes
    pub fn cast_vote(&mut self, board_fen: String, vote_fen: String) {
        let player_address = env::signer_account_id();
        // verify voter has correct board state
        require!(board_fen == self.fen_state);
        // verify player hasn't voted this period
        require!(!self.voted_this_period.contains(&player_address));
        // verify player in color to move
        let players_to_move = match self.fen_state.split(" ").nth(1) {
            Some("w") => &self.white_players,
            Some("b") => &self.black_players,
            _ => env::panic_str("malformed FEN"),
        };
        require!(players_to_move.contains(&player_address));
        // add vote to votes
        *self.votes.entry(vote_fen).or_insert(0) += 1;
    }

    // verify next vote timestamp is in the past, select winnning vote and update state
    pub fn tally_votes(&mut self) {
        // verify next vote timestamp is in the past
        require!(unix_epoch_duration().as_secs() > self.next_period_timestamp);
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
        match winning_fen.split(" ").last() {
            Some("RESIGN") => { todo!(); },
            Some("OFFER_DRAW") => { todo!(); },
            Some("other stuff") => { todo!(); },
            _ => (),
        };
        // empty votes map
        self.votes.clear();
        // empty voted_this_period set
        self.voted_this_period.clear();
        // set next vote timestamp
        self.next_period_timestamp = unix_epoch_duration().as_secs() + 600;

        todo!();
    }

    fn finish_game(&mut self, end_state: EndState) {
        // mark game as over
        self.game_active = false;
        // take 5% out of pot for dao
        todo!();
        // distribute remaining pot to winners
        todo!();
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
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{MockedBlockchain, EpochHeight};
    use near_sdk::{testing_env, VMContext};
    use std::collections::HashMap;
    use std::iter::FromIterator;

    // fn get_context(predecessor_account_id: AccountId) -> VMContext {
    //     get_context_with_epoch_height(predecessor_account_id, 0)
    // }

    // fn get_context_with_epoch_height(
    //     predecessor_account_id: AccountId,
    //     epoch_height: EpochHeight,
    // ) -> VMContext {
    //     VMContext {
    //         current_account_id: near_sdk::AccountId("alice_near".to_string()),
    //         signer_account_id: near_sdk::AccountId("bob_near".to_string()),
    //         signer_account_pk: vec![0, 1, 2],
    //         predecessor_account_id,
    //         input: vec![],
    //         block_index: 0,
    //         block_timestamp: 0,
    //         account_balance: 0,
    //         account_locked_balance: 0,
    //         storage_usage: 1000,
    //         attached_deposit: 0,
    //         prepaid_gas: near_sdk::Gas(2 * 10u64.pow(14)),
    //         random_seed: vec![0, 1, 2],
    //         is_view: false,
    //         output_data_receivers: vec![],
    //         epoch_height,
    //         view_config: todo!(),
    //     }
    // }

    // #[test]
    // // test_add_player
    // fn test_add_player() {
    //     let context = get_context("bob.near".to_string());

    //     let side = match unix_epoch_duration().as_nanos() & 1 {  // good enough for government work
    //         0 => &mut self.white_players,
    //         1 => &mut self.black_players,
    //         _ => unreachable!(),
    //     };

    //     assert_eq!("bob.near".to_string());

    // }

    


}
