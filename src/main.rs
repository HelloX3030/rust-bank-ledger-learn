mod account;

mod bank;
use bank::Bank;

fn main() {
    let mut bank = Bank::new();
    bank.add_account(String::from("Spaten"));
    bank.add_account(String::from("Spaten 2"));
    bank.add_account(String::from("Spaten 3"));
    bank.add_account(String::from("5555"));
    bank.add_account(String::from("5734958234"));
    bank.print();
}
