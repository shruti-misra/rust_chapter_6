# Mini Digital Wallet API

A lightweight Rust console application that simulates a backend API component for managing user balances and processing payment transactions safely.

## Learning Objectives

This project serves as a practical exercise to combine core Rust principles:
* **Ownership & Borrowing (Chapter 4)**: Utilizing mutable references (`&mut self`) for state modification and moving full ownership of values into methods.
* **Structs (Chapter 5)**: Grouping related data types into custom data schemas.
* **Enums & Pattern Matching (Chapter 6)**: Handling multiple distinct business operations safely without risk of runtime null-pointer exceptions by utilizing built-in type paradigms like `Option<T>`.

---

## Architecture & Core Components

### 1. `Transaction` Enum
Represents different transaction formats with associated data payloads:
* `Deposit(Option<String>, f64)`: Handles incoming funds. The description is wrapped in an `Option` type, meaning transactions can be processed with a reference (`Some(String)`) or completely anonymously (`None`).
* `Withdraw(f64)`: Captures a localized cash removal attempt.
* `Transfer(String, f64)`: Records a heap-allocated recipient identifier along with the intended payment amount.

### 2. `Wallet` Struct
A state-tracking schema containing two critical elements:
* `owner`: A heap-allocated `String` tracking the wallet holder's name.
* `balance`: An `f64` precise numerical field tracking current funds.

### 3. API Methods
* `Wallet::new(name: String) -> Self`: An associated constructor function that takes ownership of a configuration string and initializes the account balance to `0.0`.
* `process_transaction(&mut self, transaction: Transaction)`: The orchestration method. It consumes full ownership of the incoming transaction enum and mutates individual field states safely through pattern matching rules.
* `process_transfers_withdrawls(&mut self, amt: f64)`: An internal ledger guard that ensures a user cannot overdraft their balance.

---


---

## Expected Output Simulation

When executing the sample simulation inside `main()`, your terminal output will accurately mirror the following financial lifecycle:

```text
X's wallet is: Wallet { owner: "X", balance: 0.0 }
A deposit of \$100 has been made for paycheck
Your new wallet is Wallet { owner: "X", balance: 100.0 }
Error: Your balance (\$100.00) is lower than the requested amount (\$200.00)
Your wallet is Wallet { owner: "X", balance: 100.0 }
A deposit of \$500 has been made
Your new wallet is Wallet { owner: "X", balance: 600.0 }
Processing your transfer to sister...
Your wallet is Wallet { owner: "X", balance: 400.0 }
```
