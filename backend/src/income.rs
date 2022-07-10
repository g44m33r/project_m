struct AddIncomeRequest {
    amount: i32,
    category: String,
    account: String,
}

struct RemoveIncomeRequest{
    id: i64,
}

#[post("/income")]
async fn add_income(req: actix_web::web::Json<AddIncomeRequest>) -> impl Responder {
    Ok()
}
