struct CalculateLeftoverRequest {
    month: i32,
    year: i32,
}

struct CalculateSavingsRequest {
    id: i64,
    timeframe_modifier: i32,
    timeframe: TimeFrame,
}

struct ShowExpensesRequest {
    week: i32,
}

enum TimeFrame {
    Daily,
    Weekly,
    Monthly,
    Yearly
}
