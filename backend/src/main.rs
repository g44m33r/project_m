#![allow(dead_code)]

use actix_web::{web, App, HttpServer};

pub mod authentication {
    struct LoginRequest {}
    struct LogourRequest {}
}

pub mod calculations {
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
        Yearly,
    }
}

pub mod expense {
    struct AddExpenseRequest {
        amount: i32,
        category: String,
        account: String,
    }

    struct AddFutureExpenseRequest {
        amount: i32,
        category: String,
        account: String,
        date: chrono::Date<chrono::Utc>,
    }

    struct RemoveExpense {
        amount: i64,
    }
}

pub mod income {
    use actix_web::{web, HttpResponse, Responder};
    use serde::{Deserialize, Serialize};

    pub const API_URL_ADD_INCOME: &str = "/income";
    pub const API_URL_REMOVE_INCOME: &str = "/income";

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

    #[derive(Debug, Deserialize, Serialize)]
    pub struct RemoveIncomeRequest {
        pub id: i64,
    }

    pub async fn add_income(req: web::Json<AddIncomeRequest>) -> impl Responder {
        let res = AddIncomeResponse {
            code: 9000,
            message: format!("Incoming request was: {:?}", req.into_inner()),
        };
        HttpResponse::Ok().json(res)
    }

    pub async fn remove_income(_req: web::Json<RemoveIncomeRequest>) -> impl Responder {
        HttpResponse::NoContent()
    }
}

pub mod recurring {
    enum RecurringTimeFrame {
        Weekly,
        Biweekly,
        Monthly,
        Quarterly,
        Yearly,
    }

    struct SetRecurringRequest {
        time_frame: RecurringTimeFrame,
        id: i64,
        spend_type: String,
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::JsonConfig::default().limit(4096))
            .route(
                income::API_URL_ADD_INCOME,
                web::post().to(income::add_income),
            )
            .route(
                income::API_URL_REMOVE_INCOME,
                web::delete().to(income::remove_income),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, web, App};

    #[actix_web::test]
    async fn test_add_income_ok() {
        let app = test::init_service(App::new().route(
            income::API_URL_ADD_INCOME,
            web::post().to(income::add_income),
        ))
        .await;

        let req_obj = income::AddIncomeRequest {
            amount: 123,
            category: String::from("foobar"),
            account: String::from("foobar"),
        };
        let req = test::TestRequest::post()
            .uri(income::API_URL_ADD_INCOME)
            .set_json(&req_obj)
            .to_request();

        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);

        let json: income::AddIncomeResponse = test::read_body_json(res).await;
        assert_eq!(json.code, 9000);
        assert_eq!(json.message, format!("Incoming request was: {req_obj:?}"));
    }

    #[actix_web::test]
    async fn test_remove_income_ok() {
        let app = test::init_service(App::new().route(
            income::API_URL_REMOVE_INCOME,
            web::delete().to(income::remove_income),
        ))
        .await;

        let req = test::TestRequest::delete()
            .uri(income::API_URL_REMOVE_INCOME)
            .set_json(income::RemoveIncomeRequest { id: 123 })
            .to_request();

        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }
}
