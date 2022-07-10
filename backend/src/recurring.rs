enum RecurringTimeFrame {
    Weekly,
    Biweekly,
    Monthly,
    Quarterly,
    Yearly
}

struct SetRecurringRequest {
    time_frame: RecurringTimeFrame,
    id: i64,
    type: String,
}
