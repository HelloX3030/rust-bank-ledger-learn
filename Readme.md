# 🧾 Bank Ledger (In-Memory) — Learning To-Do List
## ✅ 1. Project Setup
- Initialize a new Rust project (cargo new bank_ledger)
    - Set up a simple CLI entry point in main.rs

## 🧍 2. Model the Core Structs
- Define an Account struct with:
- Account ID (e.g., String or numeric)
- Balance (e.g., f64 or u64)
    - Optional: Account holder name or metadata
- Create a Bank struct that contains:
    - A HashMap mapping account IDs to Accounts

## 💸 3. Implement Account Management
- Create new accounts
- View a list of all existing accounts
- Get a single account's current balance

💰 4. Implement Transactions

Deposit to an account

Handle invalid account ID

    Reject negative deposit amounts

Withdraw from an account

Handle invalid account ID

Reject negative amounts

        Error on insufficient funds

🧪 5. Add Transaction History (Optional, More Advanced)

Track a list of deposits/withdrawals per account

    Store transaction timestamp, type (deposit/withdrawal), and amount

🛠️ 6. Error Handling

Create meaningful error messages for:

Account not found

Invalid amount

    Insufficient funds

    Use Result and Option appropriately

🧼 7. CLI Interface (Simple)

Allow user to:

Create account

View balance

Make deposit

    Make withdrawal

    Parse commands from user input or CLI args

📦 8. Polish & Extend (Optional)

Save/load account data to/from a file (e.g., JSON or CSV)

Add unique ID generation for accounts

Pretty-print account summaries

    Add unit tests for Bank methods

📚 Bonus Learning Ideas

Use enum for transaction types

Use lifetimes or references safely with borrowed data

Refactor logic using traits if the project grows