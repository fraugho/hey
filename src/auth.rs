use crate::db::get_user_auth_info;
use std::sync::Arc;
use crate::state::*;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use std::fmt;
use anyhow::Result;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct LoginForm {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub enum LoginError {
    InvalidCredentials,
    // Other variants...
}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoginError::InvalidCredentials => write!(f, "Invalid credentials"),
            // Handle other variants...
        }
    }
}

pub async fn check_login(
    form: &LoginForm,
    state: Arc<AppState>,
) -> Result<UserSession, LoginError> {
    // Try to get user authentication info from the database
    match get_user_auth_info(form, &state.db).await {
        Ok(auth_info) => {
            // Verify the password using Argon2
            let parsed_hash = PasswordHash::new(&auth_info.password)
                .map_err(|_| LoginError::InvalidCredentials)?;
            let argon2 = Argon2::default();

            let salted_client_password = form.password.clone() + &auth_info.salt;

            if argon2.verify_password(salted_client_password.as_bytes(), &parsed_hash).is_ok() {
                // Password is correct, return a UserSession
                Ok(UserSession {
                    username: form.name.to_owned(),
                })
            } else {
                Err(LoginError::InvalidCredentials)
            }
        }
        Err(_) => Err(LoginError::InvalidCredentials),
    }
}

