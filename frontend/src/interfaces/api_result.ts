export interface APIResult<T> {
    code: number;
    message: string;
    data: T;
}