use crate::db::get_user_auth_info;
use std::sync::Arc;
use crate::state::*;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use std::fmt;
use anyhow::Result;
use axum::{
    middleware::Next,
    response::{IntoResponse, Response, Redirect},
    http::Request,
    body::Body,
    extract::State,
};
use axum_extra::extract::cookie::CookieJar;

#[derive(serde::Deserialize)]
pub struct LoginForm {
    pub login: String,
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
                    username: form.login.to_owned(),
                })
            } else {
                Err(LoginError::InvalidCredentials)
            }
        }
        Err(_) => Err(LoginError::InvalidCredentials),
    }
}

pub async fn require_auth(
    cookie_jar: CookieJar,
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Response {
    // Check if there's a session cookie
    if let Some(session_cookie) = cookie_jar.get("session_id") {
        // Verify the session exists in our store
        if state.session_store.contains_key(session_cookie.value()) {
            // Session is valid, continue to the route handler
            return next.run(request).await;
        }
    }
    
    // No valid session found, redirect to login
    Redirect::to("/login").into_response()
}

pub fn validate_session(session_id: &str, state: &AppState) -> bool {
    state.session_store.contains_key(session_id)
}
