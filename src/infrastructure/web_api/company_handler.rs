use rocket_contrib::json::Json;

use crate::infrastructure::web_api::context::Context;
use crate::usecase::interactor::company_interactor::NewCompanyRequest;
use crate::usecase::presenter::company_presenter::CompanyResponse;

#[post("/companies", format = "json", data = "<company>")]
pub fn create_company(company: Json<NewCompanyRequest>, context: Context) -> Json<CompanyResponse> {
    println!("HANDLER new company {}", company.name);
    let result = context.company_interactor
        .create_company(company.0)
        .map(|comp|
            CompanyResponse {
                name: comp.name.clone(),
                description: comp.description.clone(),
                full_address: comp.address.clone(),
                abn: comp.abn.clone(),
            }
        );
    Json(result.unwrap())
}
