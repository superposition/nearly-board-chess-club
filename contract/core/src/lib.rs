use near_sdk::collections::LazyOption;
use near_sdk::borsh::{self, BorshDeserialization, BorshSerialize};
use near_sdk::{near_bindgen, Balance, Gas};

mod deploy;
mod manager;

const NEAR_PER_STORAGE: Balance = 10_000_000_000_000_000_000; //10e18;
const DEFAULT_CONTRACT: &[u8] = include_bytes!("../wasm/donation");
const TGAS:Gas = Gas(10u64.pow(12));
const NO_DEPOSIT: Balance = 0;


#[near_bindgen]
#[derive(BorshDeserialization, BorshSerialize)]
pub struct Contract {
    code: LazyOption<Vec<u8>,
}



// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
