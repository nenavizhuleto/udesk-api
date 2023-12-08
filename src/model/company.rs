use crate::{ModelController, Result};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Company {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub itn: String,
}

#[derive(Debug, Deserialize)]
pub struct CompanyForCreate {
    name: String,
    description: Option<String>,
    itn: String,
}

impl ModelController {
    pub async fn create_company(&self, company_fc: CompanyForCreate) -> Result<Company> {
        let slug = company_fc.name.to_lowercase().replace(" ", "-");
        let result = sqlx::query(
            "INSERT INTO companies (name, description, slug, itn) VALUES ($1, $2, $3, $4)",
        )
        .bind(&company_fc.name)
        .bind(&company_fc.description)
        .bind(&slug)
        .bind(&company_fc.itn)
        .execute(&self.db)
        .await?;

        let company = Company {
            id: result.rows_affected() as i64,
            name: company_fc.name,
            description: company_fc.description,
            slug,
            itn: company_fc.itn,
        };

        Ok(company)
    }

    pub async fn get_company(&self, id: i64) -> Result<Company> {
        let company: Company = sqlx::query_as("SELECT * FROM companies WHERE id = $1")
            .bind(id)
            .fetch_one(&self.db)
            .await?;
        Ok(company)
    }

    pub async fn list_companies(&self) -> Result<Vec<Company>> {
        let companies: Vec<Company> = sqlx::query_as("SELECT * FROM companies")
            .fetch_all(&self.db)
            .await?;
        Ok(companies)
    }

    pub async fn delete_company(&self, id: i64) -> Result<i64> {
        let result = sqlx::query("DELETE FROM companies WHERE id = $1")
            .bind(id)
            .execute(&self.db)
            .await?;
        Ok(result.rows_affected() as i64)
    }
}
