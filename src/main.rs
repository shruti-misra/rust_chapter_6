//Scenario: The Mini Digital Wallet APIYou will build a backend API component for 
// managing user balances and processing payment transactions.


//Transaction Enum:
// Deposit: Contains a heap-allocated Option<T> for the optional description and an f64 amount.
// Withdraw: Contains an f64 amount.
// Transfer: Contains a heap-allocated String for the recipient's username and an f64 amount.

enum Transaction {
    Deposit(Option<String>, f64),
    Withdraw(f64),
    Transfer(String, f64)
}

// Wallet Struct:Contains fields for owner (String) and balance (f64).
#[derive(Debug)]
struct Wallet {
    owner: String, 
    balance: f64
}


impl Wallet {
//Associated Constructor Function called new.
// It should take ownership of a String for the owner's name and initialize the balance to 0.0.

    fn new(name: String) -> Self {
        Self {
        owner: name,
        balance: 0.0
    }
    } 

//Processing Method (process_transaction):
// It must take a mutable reference to the wallet so it can change the balance.
// It must take complete ownership of the Transaction enum passed into it.
// Use a match expression to destructure and handle each enum variant.
// If a withdrawal or transfer amount exceeds the balance, 
// print an error message and do not change the balance

    fn process_transfers_withdrawls(&mut self, amt:f64) {
        if self.balance < amt {
            println!("Error: Your balance (${:.2}) is lower than the requested amount (${:.2})", self.balance, amt);         
        } else {
            self.balance -= amt;
        }
    }

    fn process_transaction(&mut self, transaction: Transaction) {

        match transaction {
            Transaction::Deposit(desc, amt) => {
                self.balance += amt;
                match desc {
                    Some(desc) => {
                        println!("A deposit of ${} has been made for {}", amt, desc);
                    }
                    None => {
                        println!("A deposit of ${} has been made", amt);
                    }
                }
                println!("Your new wallet is {self:?}");
            }
            Transaction::Withdraw(amt) => {
                self.process_transfers_withdrawls(amt);
                println!("Your wallet is {self:?}");
            }
            Transaction::Transfer(recipient, amt) => {
                println!("Processing your transfer to {}...", recipient);
                self.process_transfers_withdrawls(amt);
                println!("Your wallet is {self:?}");
            }
        }

    } 
}

fn main() {

    //Initialize a new wallet under my name
    let my_name = String::from("X");
    let mut my_wallet = Wallet::new(my_name);
    println!("{}'s wallet is: {my_wallet:?}", my_wallet.owner);

    //Add a deposit to my wallter
    let deposit_amt = Transaction::Deposit(Some(String::from("paycheck")), 100.0);
    my_wallet.process_transaction(deposit_amt);

    let withdraw_amt = Transaction::Withdraw(200.0);
    my_wallet.process_transaction(withdraw_amt);

    let deposit_amt = Transaction::Deposit(None, 500.0);
    my_wallet.process_transaction(deposit_amt);

    let transfer_amt = Transaction::Transfer(String::from("sister"), 200.0);
    my_wallet.process_transaction(transfer_amt);

}
