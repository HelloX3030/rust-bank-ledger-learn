
pub struct Account {
    pub id: u64,
    pub name: String,
    pub balance: f64,
}

impl Account {
    pub fn new(name: String) -> Self {
        Account { id: 0, name, balance: 0.0 }
    }

    pub fn print(&self){
        println!("id: {}", self.id);
        println!("name: {}", self.name);
        println!("balance: {}", self.balance);
    }
}
