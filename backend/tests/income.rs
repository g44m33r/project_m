use actix_web::{http::StatusCode, test, web, App};

use project_m::income;

#[actix_web::test]
async fn test_get_income_ok() {
    let app = test::init_service(
        App::new().route(income::API_URL_INCOME, web::get().to(income::get_income)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(income::API_URL_INCOME)
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status(), StatusCode::OK);

    let json_res: income::GetIncomeResponse = test::read_body_json(res).await;
    assert_eq!(json_res.amount, 42);
}

#[actix_web::test]
async fn test_add_income_ok() {
    let app = test::init_service(
        App::new().route(income::API_URL_INCOME, web::post().to(income::add_income)),
    )
    .await;

    let req_obj = income::AddIncomeRequest {
        amount: 123,
        category: String::from("foobar"),
        account: String::from("foobar"),
    };
    let req = test::TestRequest::post()
        .uri(income::API_URL_INCOME)
        .set_json(&req_obj)
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status(), StatusCode::OK);

    let json_res: income::AddIncomeResponse = test::read_body_json(res).await;
    assert_eq!(json_res.code, 9000);
    assert_eq!(
        json_res.message,
        format!("Incoming request was: {req_obj:?}")
    );
}

#[actix_web::test]
async fn test_remove_income_ok() {
    let app = test::init_service(App::new().route(
        income::API_URL_INCOME,
        web::delete().to(income::remove_income),
    ))
    .await;

    let req = test::TestRequest::delete()
        .uri(income::API_URL_INCOME)
        .set_json(income::RemoveIncomeRequest { id: 123 })
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status(), StatusCode::NO_CONTENT);
}
