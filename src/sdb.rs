use crate::auth::LoginForm;
use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Mem;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

pub async fn init_db() ->  surrealdb::Result<()> {
    // Create database connection
    let db = Surreal::new::<Mem>(()).await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    return Ok(());
}

pub struct User {
    name: String,
    email: String,
    password: String,
}

pub async fn get_user_auth_info(form: LoginForm, db: &Surreal<Client>) -> surrealdb::Result<()> {

    //respone.0 password & response.1 salt

    let response = db
        .query("SELECT password, salt FROM users WHERE email = $email or name = $name")
        .bind(("email", form.email.clone()))
        .bind(("name", form.name.clone()))
        .await?;

    let result = (response.take(0) , response.take(1));

    return Ok(());
}
