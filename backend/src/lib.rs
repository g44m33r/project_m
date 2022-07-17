#![allow(dead_code)]

//! Crate containing all of the business logic of the application.

pub mod authentication {
    pub struct LoginRequest {}
    pub struct LogoutRequest {}
}

pub mod calculations {
    pub struct CalculateLeftoverRequest {
        pub month: i32,
        pub year: i32,
    }

    pub struct CalculateSavingsRequest {
        pub id: i64,
        pub timeframe_modifier: i32,
        pub timeframe: TimeFrame,
    }

    pub struct ShowExpensesRequest {
        pub week: i32,
    }

    pub enum TimeFrame {
        Daily,
        Weekly,
        Monthly,
        Yearly,
    }
}

pub mod expense {
    pub struct AddExpenseRequest {
        pub amount: i32,
        pub category: String,
        pub account: String,
    }

    pub struct AddFutureExpenseRequest {
        pub amount: i32,
        pub category: String,
        pub account: String,
        pub date: chrono::Date<chrono::Utc>,
    }

    pub struct RemoveExpense {
        pub amount: i64,
    }
}

pub mod income {
    use actix_web::{web, HttpResponse, Responder};
    use serde::{Deserialize, Serialize};

    pub const API_URL_INCOME: &str = "/income";

    #[derive(Deserialize, Serialize)]
    pub struct GetIncomeResponse {
        pub amount: i32,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct AddIncomeRequest {
        pub amount: i32,
        pub category: String,
        pub account: String,
    }
    #[derive(Deserialize, Serialize)]
    pub struct AddIncomeResponse {
        pub code: i32,
        pub message: String,
    }

    #[derive(Deserialize, Serialize)]
    pub struct RemoveIncomeRequest {
        pub id: i64,
    }

    /// Request handler for `GET /income`
    pub async fn get_income() -> impl Responder {
        HttpResponse::Ok().json(GetIncomeResponse { amount: 42 })
    }

    /// Request handler for `POST /income`
    pub async fn add_income(req: web::Json<AddIncomeRequest>) -> impl Responder {
        HttpResponse::Ok().json(AddIncomeResponse {
            code: 9000,
            message: format!("Incoming request was: {:?}", req.into_inner()),
        })
    }

    /// Request handler for `DELETE /income`
    pub async fn remove_income(_req: web::Json<RemoveIncomeRequest>) -> impl Responder {
        HttpResponse::NoContent().finish()
    }
}

pub mod recurring {
    pub enum RecurringTimeFrame {
        Weekly,
        Biweekly,
        Monthly,
        Quarterly,
        Yearly,
    }

    pub struct SetRecurringRequest {
        pub time_frame: RecurringTimeFrame,
        pub id: i64,
        pub spend_type: String,
    }
}
