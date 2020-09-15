use crate::domain::company::dao::dao_port::CompanyDao;
use crate::domain::company::model::company::Company;
use std::error::Error;
use chrono::{Utc};
use serde::{Deserialize};

pub struct CompanyInteractor {
    pub dao: Box<dyn CompanyDao>
}

impl CompanyInteractor {
    pub fn create_company(&self, new_company: NewCompanyRequest) -> Result<Company, Box<dyn Error>> {
        Ok(Company {
            id: Option::Some(1),
            name: String::from("name"),
            description: String::from("description"),
            address: String::from("address"),
            post_code: 2000,
            city: String::from("Sydney"),
            country: String::from("Aus"),
            abn: Option::None,
            ceo_id: Option::None,
            created_at: Utc::now().naive_utc()
        })
    }
}


#[derive(Debug, Clone, Deserialize)]
pub struct NewCompanyRequest {
    pub name: String,
    pub description: String,
    pub address: String,
    pub post_code: i8,
    pub city: String,
    pub country: String,
    pub abn: String,
}
