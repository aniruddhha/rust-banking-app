use crate::bank_db::account;
use crate::bank_db::account::Account;
use crate::bank_db::bank_account::BankAccount;
use crate::bank_demo::user_ip;

fn display_menu() {
    println!("Welcome to the Bank Demo!");
    println!("Please select an option:");
    println!("1. Create Account");
    println!("2. Deposit");
    println!("3. Withdraw");
    println!("4. Check Balance");
    println!("5. Exit");
}

pub fn continious_menu() {
    let mut ac_op: Option<Account> = None;

    loop {
        display_menu();
        let choice = user_ip::read_number("Enter your choice:", "Bad Choice").unwrap();

        match choice {
          
            1 => {
                println!("Lets Create Account");

                let id = user_ip::read_number("Enter account id:", "Invalid User Id").unwrap();
                let balance: f64 = user_ip::read_number("Enter initial balance:", "Invalid Balance").unwrap();

                ac_op = Some(account::Account::new(id, balance));

                if let Some(ref mut account) = ac_op {
                    account.set_id(id);
                    account.set_balance(balance);

                    println!("Account created with ID: {} and balance: {}", account.get_id(), account.get_balance());
                } else {
                    println!("Failed to create account.");
                }
            },
            2 => {
                let amount: f64 = user_ip::read_number("Enter Deposit Amount: ", "Invalid Amount").unwrap();
                if let Some(ref mut account) = ac_op {
                    account.deposit(amount);
                    println!("Deposited {}. New balance: {}", amount, account.get_balance());
                } else {
                    println!("No account found. Please create an account first.");
                }
            },
            3 =>  {
                let amount: f64 = user_ip::read_number("Enter Withdraw Amount: ", "Invalid Amount").unwrap();
                if let Some(ref mut account) = ac_op {
                    match account.withdraw(amount) {
                        Ok(_) => println!("Withdrew {}. New balance: {}", amount, account.get_balance()),
                        Err(e) => println!("Error: {}", e),
                    }
                } else {
                    println!("No account found. Please create an account first.");
                }
            },
            4 => {
                if let Some(ref account) = ac_op {
                    println!("Current balance: {}", account.get_balance());
                } else {
                    println!("No account found. Please create an account first.");
                }
            }
            _ => {
                println!("Exiting...");
                break;
            }
        }
    }
}