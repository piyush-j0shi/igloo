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
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    input.trim().to_string()
}

fn main() {
    println!("enter name : ");
    let user_name = read_input();

    println!("enter inital balance : ");
    let user_balance = read_input().parse().expect("not a number");

    let mut user = Account::new_account(user_name, user_balance);
    user.get_details();

    user.withdraw(100);
    user.get_details();

    user.deposite(100);
    user.get_details();
}
