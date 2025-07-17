use std::collections::HashMap;
use crate::account::Account;

pub struct Bank{
    next_id: u64,
    pub accounts: HashMap<u64, Account>,
}

impl Bank {
    pub fn new() -> Self{
        Bank { next_id: 1, accounts: HashMap::new() }
    }

    pub fn print(&self) {
        println!("==================================================");
        if self.accounts.is_empty() {
            println!("No Accounts Found!");
        }
        for i in 0..self.next_id{
            if let Some(account) = self.accounts.get(&i) {
                account.print();
            }
        }
    }

    pub fn add_account(&mut self, name: String) {
        let mut acc = Account::new(name);
        acc.id = self.next_id;
        self.next_id += 1;
        self.accounts.insert(acc.id, acc);
    }

    pub fn get_id(&self, name: String) -> Option<u64>
    {
        for (id, account) in &self.accounts {
            if account.name == name {
                return Some(*id);
            }
        }
        None
    }

    pub fn delete_account(&mut self, name: String) {
        if let Some(id) = self.get_id(name) {
            self.accounts.remove(&id);
        }
    }

    pub fn delete_account_id(&mut self, id: u64) {
        self.accounts.remove(&id);
    }
}
