import { APIResult } from "./api_result";

export interface Income {

    add_income(
        amount: number, 
        category: string, 
        account: string
        ): APIResult<boolean>;

    remove_income(
        id: number
        ): APIResult<boolean>;

}