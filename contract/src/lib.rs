/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use std::collections::{HashMap, HashSet};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, require, AccountId, ONE_NEAR};

// Define the default message
const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const VOTING_PERIOD_INTERVAL: u64 = 30;

enum GameState {
    ACTIVE,
    DRAWOFFER,
    WHITEWIN,
    BLACKWIN,
    WHITERESIGN,
    BLACKRESIGN,
    STALEMATE,
    DRAW,
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    game_active: bool,
    board_fen: String,
    buyin_amount: u128,
    base_voting_interval: u64,
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
            board_fen: STARTING_POSITION.to_string(),
            buyin_amount: ONE_NEAR,
            base_voting_interval: VOTING_PERIOD_INTERVAL,
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
    pub fn new(buyin_amount: u128, base_voting_interval: u64) -> Self {
        Self { buyin_amount, base_voting_interval, ..Default::default() }
    }
    pub fn get_fen(self) -> String {
        self.board_fen
    }

    #[payable]
    pub fn add_player(&mut self, player_address: AccountId) {
        // transfer buy-in to contract
        require!(env::attached_deposit() >= self.buyin_amount, "send more coin lol");  // using payable function
        // add player to random color
        if self.white_players.is_empty() {
            self.white_players.insert(player_address);
        } else if self.black_players.is_empty() {
            self.black_players.insert(player_address);
        } else {
            let side = match env::block_timestamp_ms() % 2 == 0 {  // good enough for government work
                true => &mut self.white_players,
                false => &mut self.black_players,
            };
            side.insert(player_address);
        }
    }

    // add vote to current period votes
    pub fn cast_vote(&mut self, board_fen: String, vote_fen: String) {
        let player_address = env::signer_account_id();
        // verify voter has correct board state
        require!(board_fen == self.board_fen, "out of date board state");
        // verify player has voting rights
        require!(self.voting_rights(&player_address), "player can't vote in this game");
        // verify player hasn't voted this period
        require!(!self.voted_this_period.contains(&player_address), "player already voted this period");
        // verify player in color to move
        require!(self.can_vote(&player_address), "it's not your turn");
        // add vote to votes
        *self.votes.entry(vote_fen).or_insert(0) += 1;
        // add player to voted this period
        self.voted_this_period.insert(player_address);
    }

    pub fn voting_rights(&self, player_address: &AccountId) -> bool {
        self.white_players.contains(player_address) || self.black_players.contains(player_address)
    }

    pub fn can_vote(&self, player_address: &AccountId) -> bool {
        let players_to_move = match self.board_fen.split(" ").nth(1) {
            Some("w") => &self.white_players,
            Some("b") => &self.black_players,
            _ => env::panic_str("malformed FEN"),
        };
        players_to_move.contains(player_address) || !self.voted_this_period.contains(player_address)
    }

    // verify next vote timestamp is in the past, select winnning vote and update state
    pub fn tally_votes(&mut self) {
        // verify next vote timestamp is in the past
        require!(env::block_timestamp() > self.next_period_timestamp, "current voting period not over");
        // find highest voted fen
        let mut curr_most_votes = 0;
        let mut winning_fen = &self.board_fen;
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
        self.board_fen = winning_fen.to_owned();
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

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".parse().unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn board_fen_should_be_starting_fen() {
        let context = get_context(true);
        testing_env!(context);
        // ... Write test here
        let contract = Contract::new(ONE_NEAR, VOTING_PERIOD_INTERVAL);
        assert_eq!(
            contract.get_fen(),
            STARTING_POSITION.to_string(),
        );
    }

    #[test]
    #[should_panic(expected = "send more coin")]
    fn bob_cannot_buyin_without_deposit() {
        let context = get_context(false);
        testing_env!(context);
        // ... Write test here
        let bob: AccountId = "bob_near".parse().unwrap();
        let mut contract = Contract::new(ONE_NEAR, VOTING_PERIOD_INTERVAL);
        
        contract.add_player(bob.clone());
    }
}
