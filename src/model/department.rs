use crate::{ModelController, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Department {
    pub id: i32,
    pub company_id: i32,
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
        company_id: i32,
        dep_fc: DepartmentForCreate,
    ) -> Result<Department> {
        let result = sqlx::query!(
            "INSERT INTO departments (company_id, name, description, address, contacts) VALUES (?, ?, ?, ?, ?)",
            company_id,
            &dep_fc.name,
            &dep_fc.description,
            &dep_fc.address,
            &dep_fc.contacts,
            ).execute(&self.db).await?;

        let dep = Department {
            id: result.last_insert_id() as i32,
            company_id,
            name: dep_fc.name,
            description: dep_fc.description,
            address: dep_fc.address,
            contacts: dep_fc.contacts,
        };

        Ok(dep)
    }
    pub async fn get_department(&self, id: i32) -> Result<Department> {
        let department = sqlx::query_as!(Department, "SELECT * FROM departments WHERE id = ?", id)
            .fetch_one(&self.db)
            .await?;
        Ok(department)
    }

    pub async fn list_departments(&self) -> Result<Vec<Department>> {
        let departments = sqlx::query_as!(Department, "SELECT * FROM departments")
            .fetch_all(&self.db)
            .await?;
        Ok(departments)
    }

    pub async fn list_departments_by_company(&self, company_id: i32) -> Result<Vec<Department>> {
        let departments = sqlx::query_as!(
            Department,
            "SELECT * FROM departments WHERE company_id = ?",
            company_id
        )
        .fetch_all(&self.db)
        .await?;
        Ok(departments)
    }

    pub async fn delete_department(&self, id: i32) -> Result<i32> {
        let result = sqlx::query!("DELETE FROM departments WHERE id = ?", id)
            .execute(&self.db)
            .await?;
        Ok(result.rows_affected() as i32)
    }
}
