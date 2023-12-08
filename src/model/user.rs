use crate::{ModelController, Result};
use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum Role {
    #[serde(rename = "employee")]
    Employee,
    #[serde(rename = "executor")]
    Executor,
    #[serde(rename = "admin")]
    Admin,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub role: Role,
}

#[derive(Debug, Deserialize)]
pub struct UserForCreate {
    username: String,
    password: String,
    first_name: String,
    last_name: String,
}

#[cfg(feature = "mysql")]
impl ModelController {
    pub async fn create_user(&self, user_fc: UserForCreate, role: Role) -> Result<User> {
        let result = sqlx::query(
            "INSERT INTO users (username, password, first_name, last_name, role) VALUES ($1, $2, $3, $4, $5)")
            .bind(&user_fc.username)
            .bind(&user_fc.password)
            .bind(&user_fc.first_name)
            .bind(&user_fc.last_name)
            .bind(&role)
            .execute(&self.db).await?;

        let id = result.rows_affected() as i64;

        let user = User {
            id,
            username: user_fc.username,
            password: user_fc.password,
            first_name: user_fc.first_name,
            last_name: user_fc.last_name,
            role,
        };
        Ok(user)
    }

    pub async fn get_user(&self, id: i64) -> Result<User> {
        let user: User = sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&self.db)
            .await?;
        Ok(user)
    }

    pub async fn list_users(&self) -> Result<Vec<User>> {
        let users: Vec<User> = sqlx::query_as("SELECT * FROM users")
            .fetch_all(&self.db)
            .await?;
        Ok(users)
    }
    pub async fn list_employees(&self) -> Result<Vec<User>> {
        let users: Vec<User> = sqlx::query_as("SELECT * FROM users WHERE role = $1")
            .bind(Role::Employee)
            .fetch_all(&self.db)
            .await?;
        Ok(users)
    }

    pub async fn list_executors(&self) -> Result<Vec<User>> {
        let users: Vec<User> = sqlx::query_as("SELECT * FROM users WHERE role = $1")
            .bind(Role::Executor)
            .fetch_all(&self.db)
            .await?;
        Ok(users)
    }

    pub async fn list_admins(&self) -> Result<Vec<User>> {
        let users: Vec<User> = sqlx::query_as("SELECT * FROM users WHERE role = $1")
            .bind(Role::Admin)
            .fetch_all(&self.db)
            .await?;
        Ok(users)
    }
    pub async fn delete_user(&self, id: i64) -> Result<i64> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.db)
            .await?;
        Ok(result.rows_affected() as i64)
    }
}
