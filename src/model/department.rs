use crate::{ModelController, Result};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Department {
    pub id: i64,
    pub company_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub address: String,
    pub contacts: String,
}

#[derive(Debug, Deserialize)]
pub struct DepartmentForCreate {
    pub name: String,
    pub description: Option<String>,
    pub address: String,
    pub contacts: String,
}

impl ModelController {
    pub async fn create_department(
        &self,
        company_id: i64,
        dep_fc: DepartmentForCreate,
    ) -> Result<Department> {
        let result = sqlx::query(
            "INSERT INTO departments (company_id, name, description, address, contacts) VALUES ($1, $2, $3, $4, $5)")
            .bind(company_id)
            .bind(&dep_fc.name)
            .bind(&dep_fc.description)
            .bind(&dep_fc.address)
            .bind(&dep_fc.contacts).execute(&self.db).await?;

        let dep = Department {
            id: result.rows_affected() as i64,
            company_id,
            name: dep_fc.name,
            description: dep_fc.description,
            address: dep_fc.address,
            contacts: dep_fc.contacts,
        };

        Ok(dep)
    }
    pub async fn get_department(&self, id: i64) -> Result<Department> {
        let department: Department = sqlx::query_as("SELECT * FROM departments WHERE id = $1")
            .bind(id)
            .fetch_one(&self.db)
            .await?;
        Ok(department)
    }

    pub async fn list_departments(&self) -> Result<Vec<Department>> {
        let departments: Vec<Department> = sqlx::query_as("SELECT * FROM departments")
            .fetch_all(&self.db)
            .await?;
        Ok(departments)
    }

    pub async fn list_departments_by_company(&self, company_id: i64) -> Result<Vec<Department>> {
        let departments: Vec<Department> =
            sqlx::query_as("SELECT * FROM departments WHERE company_id = $1")
                .bind(company_id)
                .fetch_all(&self.db)
                .await?;
        Ok(departments)
    }

    pub async fn delete_department(&self, id: i64) -> Result<i64> {
        let result = sqlx::query("DELETE FROM departments WHERE id = $1")
            .bind(id)
            .execute(&self.db)
            .await?;
        Ok(result.rows_affected() as i64)
    }
}
