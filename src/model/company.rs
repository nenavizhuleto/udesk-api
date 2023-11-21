use crate::{ModelController, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Company {
    pub id: i32,
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
        let result = sqlx::query!(
            "INSERT INTO companies (name, description, slug, itn) VALUES (?, ?, ?, ?)",
            &company_fc.name,
            &company_fc.description,
            &slug,
            &company_fc.itn,
        )
        .execute(&self.db)
        .await?;

        let company = Company {
            id: result.last_insert_id() as i32,
            name: company_fc.name,
            description: company_fc.description,
            slug,
            itn: company_fc.itn,
        };

        Ok(company)
    }

    pub async fn get_company(&self, id: i32) -> Result<Company> {
        let company = sqlx::query_as!(Company, "SELECT * FROM companies WHERE id = ?", id)
            .fetch_one(&self.db)
            .await?;
        Ok(company)
    }

    pub async fn list_companies(&self) -> Result<Vec<Company>> {
        let companies = sqlx::query_as!(Company, "SELECT * FROM companies")
            .fetch_all(&self.db)
            .await?;
        Ok(companies)
    }

    pub async fn delete_company(&self, id: i32) -> Result<i32> {
        let result = sqlx::query!("DELETE FROM companies WHERE id = ?", id)
            .execute(&self.db)
            .await?;
        Ok(result.rows_affected() as i32)
    }
}
