use sqlx::PgPool;
use crate::auth::LoginForm;
use sqlx::{Pool, Postgres};
use sqlx::postgres::{PgPoolOptions, PgRow};

pub async fn init_db() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://casaos:casaos@11.20.44.16:5432/postgres")
        .await?;

    // Drop all tables
    let drop_tables = sqlx::query(r#"
        DO $$ DECLARE
            r RECORD;
        BEGIN
            FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = current_schema()) LOOP
                EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(r.tablename) || ' CASCADE';
            END LOOP;
        END $$;
    "#);
    drop_tables.execute(&pool).await?;


    // Recreate your schema
    let create_schema = sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS messages (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS example (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL
        );
    "#);

    create_schema.execute(&pool).await?;

    Ok(pool)
}

pub struct User {
    name: String,
    email: String,
    password: String,
}

#[derive(sqlx::FromRow)]
struct Password { pub password: String, pub salt: String }

pub async fn get_user_auth_info(form: &LoginForm, pool: &Pool<Postgres>) -> Result<Password, sqlx::Error> {
    sqlx::query_as::<_, Password>(
        "SELECT password, salt FROM users WHERE email = $1 OR name = $2"
    )
    .bind(&form.email)
    .bind(&form.name)
    .fetch_one(pool)
    .await
}
