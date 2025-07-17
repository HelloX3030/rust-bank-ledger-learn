mod account;

mod bank;
use bank::Bank;

fn main() {
    let mut bank = Bank::new();
    bank.print();
    bank.add_account(String::from("Spaten"));
    bank.add_account(String::from("Spaten 2"));
    bank.add_account(String::from("Spaten 3"));
    bank.add_account(String::from("5555"));
    bank.add_account(String::from("5734958234"));
    bank.deposit("Spaten", 11111);
    if let Err(e) = bank.withdrawl("Spaten", 10000000){
        println!("{}", e);
    }
    bank.print();
    bank.delete_account_id(1);
    bank.delete_account("5555");
    bank.print();
}
