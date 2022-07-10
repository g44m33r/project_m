import { APIResult } from "./api_result";

export interface Authentication {

    login(): APIResult<string>;

    logout(): APIResult<boolean>;

}