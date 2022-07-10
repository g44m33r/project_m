import { APIResult } from "./api_result";

export interface Recurring {

    set_recurring(
        recurring_time_frame: RecurringTimeFrame, 
        id: number, 
        type?: "income"|"expense"
    ): APIResult<boolean>
    
}

enum RecurringTimeFrame {
    WEEKLY = 0,
    BIWEEKLY = 1,
    MONTHLY = 2,
    QUARTERLY = 3,
    YEARLY = 4,
}