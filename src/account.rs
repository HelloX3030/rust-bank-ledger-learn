
pub struct Account {
    pub id: i64,
    pub name: String,
    pub balance: f64,
}

impl Account {
    pub fn new(id: i64, name: String, balance: f64) -> Self {
        Account { id, name, balance }
    }

    pub fn print(&self){
        println!("id: {}", self.id);
        println!("name: {}", self.name);
        println!("balance: {}", self.balance);
    }
}
