// std Moduls
use std::io;

// Custom Moduls
mod account;

mod bank;
use bank::Bank;

use std::io::Write;

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line!");

    return user_input.trim().to_string();
}

fn main() {
    let mut bank = Bank::new();
    let mut user_input = String::new();

    loop {
        user_input = get_user_input("> ");
        let command: &str = &user_input;

        match command {
            "exit" => break,
            "print" => bank.print(),
            "add" => {
                let name = get_user_input("name: ");
                bank.add_account(name);
            },
            "delete" => {
                let name = get_user_input("name: ");
                bank.delete_account(&name);
            },
            "deposit" => {
                let name = get_user_input("name: ");
                let input = get_user_input("amount: ");
                let amount: u64 = match input.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid number entered!");
                        return;
                    }
                };
                bank.deposit(&name, amount);
            },
            "withdrawl" => {
                let name = get_user_input("name: ");
                let input = get_user_input("amount: ");
                let amount: u64 = match input.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid number entered!");
                        return;
                    }
                };
                if let Err(e) = bank.withdrawl(&name, amount){
                    println!("{}", e);
                }
            }
            _ => println!("Unknown command: {}", command),
        }
    }
    
    // bank.print();
    // bank.add_account(String::from("Spaten"));
    // bank.add_account(String::from("Spaten 2"));
    // bank.add_account(String::from("Spaten 3"));
    // bank.add_account(String::from("5555"));
    // bank.add_account(String::from("5734958234"));
    // bank.deposit("Spaten", 11111);
    // if let Err(e) = bank.withdrawl("Spaten", 10000000){
    //     println!("{}", e);
    // }
    // bank.print();
    // bank.delete_account_id(1);
    // bank.delete_account("5555");
    // bank.print();
}
