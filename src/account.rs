
pub struct Account {
    pub id: u64,
    pub name: String,
    pub balance: u64,
}

impl Account {
    pub fn new(name: String) -> Self {
        Account { id: 0, name, balance: 0 }
    }

    pub fn print(&self){
        println!("id: {}", self.id);
        println!("name: {}", self.name);
        println!("balance: {}", self.balance);
    }

    pub fn deposit(&mut self, amount: u64) {
        self.balance += amount;
        println!("{} has deposit {} Money", self.name, amount);
    }

    pub fn withdrawl(&mut self, amount: u64) -> Result<(), String> {
        if amount > self.balance {
            return Err("Withdrawl for ".to_string() + &self.name + " failed: Not enough funds!");
        }
        self.balance -= amount;
        println!("{} Has withdrawn {} Money", self.name, amount);
        Ok(())
    }    
}
