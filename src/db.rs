use sqlx::PgPool;

pub async fn init_db() -> Result<PgPool, sqlx::Error> {
    let database_url = "postgres://casaos:casaos@11.20.44.16:5432/postgres";
    let pool = PgPool::connect(database_url).await?;

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
struct Password { password: String,salt: String }

pub async fn get_user(form: User, db: Pool<Postgres>) -> Result<Password, sqlx::Error> {
    sqlx::query_as::<_, Password>("SELECT password, salt FROM users WHERE email = ? OR name = ?")
        .bind(form.email)
        .bind(form.name)
        .fetch_optional(&mut db)
        .await
}
/*
pub async fn init_db() -> Result<PgPool, sqlx::Error> {
PgPool::connect("http://11.20.44.16:5432").await
}
*/
