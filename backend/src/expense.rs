struct AddExpenseRequest {
    amount: i32,
    category: String,
    account: String,
}

struct AddFutureExpenseRequest {
    amount: i32,
    category: String,
    account: String,
    date: chrono::Date,
}

struct RemoveExpense {
    amount: i64,
}
