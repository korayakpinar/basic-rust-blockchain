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
        &self.accounts.get(&address).unwrap().balance
    }

    pub fn get_nonce(&self, address: String) -> &u64 {
        &self.accounts.get(&address).unwrap().nonce
    }

    pub fn get_address(&self, address: String) -> &String {
        &self.accounts.get(&address).unwrap().address
    }
}

struct Account {
    balance: u64,
    nonce: u64,
    address: String,
}