use crate::{ModelController, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Role {
    #[serde(rename = "employee")]
    Employee,
    #[serde(rename = "executor")]
    Executor,
    #[serde(rename = "admin")]
    Admin,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_str = match self {
            Role::Employee => "employee",
            Role::Executor => "executor",
            Role::Admin => "admin",
        };
        write!(f, "{self_str}")
    }
}

impl From<String> for Role {
    fn from(value: String) -> Self {
        match value.as_str() {
            "employee" => Self::Employee,
            "executor" => Self::Executor,
            "admin" => Self::Admin,
            _ => unreachable!("unknown user role"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
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
        let result = sqlx::query!(
            "INSERT INTO users (username, password, first_name, last_name, role) VALUES (?, ?, ?, ?, ?)", 
            &user_fc.username,
            &user_fc.password,
            &user_fc.first_name,
            &user_fc.last_name,
            &role.to_string(),
            ).execute(&self.db).await?;

        let id = result.last_insert_id() as i32;

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

    pub async fn get_user(&self, id: i32) -> Result<User> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
            .fetch_one(&self.db)
            .await?;
        Ok(user)
    }

    pub async fn list_users(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as!(User, "SELECT * FROM users")
            .fetch_all(&self.db)
            .await?;
        Ok(users)
    }
    pub async fn list_employees(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE role = ?",
            Role::Employee.to_string()
        )
        .fetch_all(&self.db)
        .await?;
        Ok(users)
    }

    pub async fn list_executors(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE role = ?",
            Role::Executor.to_string()
        )
        .fetch_all(&self.db)
        .await?;
        Ok(users)
    }

    pub async fn list_admins(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE role = ?",
            Role::Admin.to_string()
        )
        .fetch_all(&self.db)
        .await?;
        Ok(users)
    }
    pub async fn delete_user(&self, id: i32) -> Result<i32> {
        let result = sqlx::query!("DELETE FROM users WHERE id = ?", id)
            .execute(&self.db)
            .await?;
        Ok(result.rows_affected() as i32)
    }
}

#[cfg(feature = "memory")]
impl ModelController {
    pub async fn create_user(&self, user_fc: UserForCreate, role: Role) -> Result<User> {
        let mut store = self.users_store.lock().unwrap();
        let id = store.len() as u64;
        let user = User {
            id,
            username: user_fc.username,
            password: user_fc.password,
            first_name: user_fc.first_name,
            last_name: user_fc.last_name,
            role,
        };
        store.push(Some(user.clone()));
        Ok(user)
    }

    pub async fn list_users(&self) -> Result<Vec<User>> {
        let store = self.users_store.lock().unwrap();
        let users = store.iter().filter_map(|u| u.clone()).collect();
        Ok(users)
    }

    pub async fn delete_user(&self, id: i32) -> Result<User> {
        let mut store = self.users_store.lock().unwrap();
        let user = store.get_mut(id as usize).and_then(|u| u.take());
        user.ok_or(Error::DeleteIdNotFound { id })
    }
}
