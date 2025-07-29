use chrono::{DateTime, Utc};
use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(1);

fn get_sequential_id() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug)]
enum Transaction {
    Deposite,
    Withdraw,
    Transfer,
}

#[derive(Debug)]
struct Transactionrecords {
    transactiontype: Transaction,
    amount: usize,
    timestamp: DateTime<Utc>,
}

#[derive(Debug)]
struct Account {
    uid: usize,
    name: String,
    balance: usize,
    transcation: Vec<Transactionrecords>,
}

impl Account {
    fn new_account(name: String, current_balance: usize) -> Self {
        let mut record: Vec<Transactionrecords> = Vec::new();
        if current_balance <= 0 {
            println!("starting balance can not be negative");

            let record_newaccount = Transactionrecords {
                transactiontype: Transaction::Deposite,
                amount: 0,
                timestamp: Utc::now(),
            };
            record.push(record_newaccount);
            Self {
                uid: get_sequential_id(),
                name: name,
                balance: 0,
                transcation: record,
            }
        } else {
            let record_newaccount = Transactionrecords {
                transactiontype: Transaction::Deposite,
                amount: current_balance,
                timestamp: Utc::now(),
            };
            record.push(record_newaccount);
            Self {
                uid: get_sequential_id(),
                name: name,
                balance: current_balance,
                transcation: record,
            }
        }
    }

    // todo : need to fix transcation: Vec<Transactionrecords> for withdraw
    fn withdraw(&mut self, amount: usize) {
        if amount > self.balance {
            println!("not sufficient fund");
        } else if amount <= 0 {
            println!("amount can not be less than 0");
        } else {
            self.balance -= amount;

            let new_item = Transactionrecords {
                transactiontype: Transaction::Withdraw,
                amount: amount,
                timestamp: Utc::now(),
            };
            self.transcation.push(new_item);
        }
    }

    fn deposite(&mut self, amount: usize) {
        if amount <= 0 {
            println!("amount should be greater than 0");
        } else {
            self.balance += amount;

            let new_item = Transactionrecords {
                transactiontype: Transaction::Deposite,
                amount: amount,
                timestamp: Utc::now(),
            };
            self.transcation.push(new_item);
        }
    }

    fn get_details(&self) {
        println!("===================================================================");

        println!("name : {}", self.name);
        println!("current balance : {}", self.balance);

        println!("transaction history");
        for item in &self.transcation {
            println!("item : {:?}", item);
        }
        println!("===================================================================");
    }

    fn self_transfer(&mut self, amount: usize, other_account: &mut Account) {
        if amount <= 0 {
            println!("value can not be negative");
        } else if amount > self.balance {
            println!("not sufficient fund");
        } else {
            self.balance -= amount;
            other_account.balance += amount;

            let self_account = Transactionrecords {
                transactiontype: Transaction::Withdraw,
                amount: amount,
                timestamp: Utc::now(),
            };

            let other_account_state = Transactionrecords {
                transactiontype: Transaction::Deposite,
                amount: amount,
                timestamp: Utc::now(),
            };

            self.transcation.push(self_account);
            other_account.transcation.push(other_account_state);
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    input.trim().to_string()
}

fn execute() {
    println!("enter name : ");
    let user_name = read_input();

    println!("enter inital balance : ");
    let user_balance = read_input().parse().expect("failed to read a number");

    let mut user = Account::new_account(user_name, user_balance);
    user.get_details();

    println!("other account");
    let mut user_1 = Account::new_account(String::from("user2"), user_balance);
    user_1.get_details();

    loop {
        println!("=====================================================================================================================");
        println!("select an operation : get : getdetails | draw :withdraw  | site : deposite | exit : exit loop | self : self transferr");
        let operation_input = read_input().trim().to_lowercase();

        if operation_input == "draw" {
            println!("enter withdraw amount");
            let draw_amount: usize = read_input().parse().expect("failed to read a number");

            user.withdraw(draw_amount);
            user.get_details();
        } else if operation_input == "site" {
            println!("enter withdraw amount");
            let site_amount: usize = read_input().parse().expect("failed to read a number");

            user.deposite(site_amount);
            user.get_details();
        } else if operation_input == "get" {
            user.get_details();
        } else if operation_input == "exit" {
            break;
        } else if operation_input == "self" {
            user.self_transfer(100, &mut user_1);
            user.get_details();
        }
    }

    user_1.get_details();
}

fn main() {
    execute();
}
