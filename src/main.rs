mod account;

use account::Account;

fn main() {
    let ac1 = Account::new(55, String::from("Spaten"), 0.0);
    ac1.print();
}
