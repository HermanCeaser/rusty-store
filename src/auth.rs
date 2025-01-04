use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::sqlite::SqlitePool;
use sqlx::Row;

use crate::util;

pub struct AuthManager {
    db: SqlitePool,
}

impl AuthManager {
    pub async fn new(database_url: &str) -> Self {
        let db = SqlitePool::connect(database_url)
            .await
            .expect("Failed to connect to DB!");
        Self { db }
    }

    pub async fn initialize(&self) {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                hashed_password TEXT NOT NULL
            );
            "#,
        )
        .execute(&self.db)
        .await
        .expect("Failed to create users table");
    }

    pub async fn create_user(&self, username: &str, password: &str) -> Result<(), String> {
        let hashed_password =
            hash(password, DEFAULT_COST).map_err(|_| "Failed to hash password")?;
        sqlx::query("INSERT INTO users (username, hashed_password) VALUES (?, ?)")
            .bind(username)
            .bind(hashed_password)
            .execute(&self.db)
            .await
            .map_err(|_| "Failed to insert user")?;
        Ok(())
    }

    pub async fn authenticate(&self, username: &str, password: &str) -> Result<(), String> {
        let row = sqlx::query("SELECT hashed_password FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(&self.db)
            .await
            .map_err(|_| "Failed to query database")?;

        if let Some(row) = row {
            let hashed_password: String = row.get(0);
            if verify(password, &hashed_password).map_err(|_| "Failed to verify password")? {
                return Ok(());
            }
        }
        Err("Invalid username or password".into())
    }
}

pub async fn login(auth_manager: &AuthManager) {
    let username = util::get_user_input("Enter your username: ");

    println!("Enter your password:");
    let password = rpassword::read_password().expect("Failed to read password"); // Secure password input

    match auth_manager.authenticate(&username, &password).await {
        Ok(_) => {
            println!("Login successful!");
            // start_store_operations().await; // Proceed to store operations
        }
        Err(e) => {
            println!("Login failed: {}", e);
            std::process::exit(1);
        }
    }
}

pub async fn register(auth_manager: &AuthManager) {
    let username = util::get_user_input("Choose a unique username: ");

    println!("Choose a password:");
    let password = rpassword::read_password().expect("Failed to read password"); // Secure password input

    match auth_manager.create_user(&username, &password).await {
        Ok(_) => {
            println!("Registration successful! You can now log in.");
            login(auth_manager).await;
        }
        Err(e) => {
            println!("Registration failed: {}", e);
            std::process::exit(1);
        }
    }
}
