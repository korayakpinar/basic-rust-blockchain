use std::collections::HashMap;
pub struct State {
    accounts: HashMap<String, Account>,
}

impl State {
   pub fn new() -> State {
        State {
            accounts: HashMap::new(),
        }
    }

    pub fn get_account(&self, address: String) -> &Account {
        &self.accounts.get(&address).unwrap()
    }

    pub fn get_balance(&self, address: String) -> &u64 {
        match self.accounts.get(&address) {
            Some(account) => &account.balance,
            None => &0,
        }
    }

    pub fn get_nonce(&self, address: String) -> &u64 {
        match self.accounts.get(&address) {
            Some(account) => &account.nonce,
            None => &0,
        }
    }

    pub fn get_address(&self, address: String) -> &str {
        match self.accounts.get(&address) {
            Some(account) => &account.address,
            None => "0",
        }
    }
}

struct Account {
    balance: u64,
    nonce: u64,
    address: String,
}

#[cfg(test)]

mod tests {
    use crate::{helpers::new_random_wallet};
    use super::State;


    #[test]
    fn test_get_account() {
        let wallet = new_random_wallet();
        assert_eq!(wallet.get_address().len(),64);
    }
    #[test]
    fn test_balance() {
        let wallet = new_random_wallet();
        let address = wallet.get_address().to_string();
        let state = State::new();
        let balance = *state.get_balance(address);
        assert_eq!(balance,0);
    }
    #[test]
    fn test_address() {
        let wallet = new_random_wallet();
        let address = wallet.get_address().to_string();
        let state = State::new();
        let address_func = state.get_address(address);
        assert_eq!(address_func,"0");
    }
}