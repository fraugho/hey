use crate::db::get_user_auth_info;
use axum::{
    extract::State
};
use std::sync::Arc;
use crate::state::AppState;

#[derive(serde::Deserialize)]
pub struct LoginForm {
    pub name: String,
    pub email: String,
    pub password: String
}

pub async fn check_login(form: LoginForm, State(state): State<Arc<AppState>>){
    //row.1 password
    //row.2 salt
    if let Ok(row) = get_user_auth_info(form, &state.db).await {

    }
}
