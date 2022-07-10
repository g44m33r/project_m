import { APIResult } from "./api_result";

export interface Expense {
    
    add_expense(
        amount: number,
        category: string,
        account: string
    ): APIResult<boolean>

    add_future_expense(
        amount: number,
        category: string,
        account: string,
        date: Date
    ): APIResult<boolean>

    remove_expense(
        id: string
    ): APIResult<boolean>

}

export interface ExpenseObject {
    amount: number;
    category: string;
    account: string;
    date: Date;
} 

export interface WeekdayExpense {
    [key: string]: {
        total: number,
        expenses: [ExpenseObject]
    }
}

export interface WeeklyExpenses {
    total: number;
    weekdays: WeekdayExpense
}