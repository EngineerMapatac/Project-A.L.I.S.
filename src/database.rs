// src/database.rs
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenvy::dotenv;

pub async fn connect_and_init() {
    // 1. Load the secrets from the .env file
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("❌ DATABASE_URL must be set in .env");

    println!("☁️ Connecting to Render PostgreSQL Database...");

    // 2. Establish the connection pool
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Successfully connected to the cloud database!");
            pool
        }
        Err(e) => {
            eprintln!("❌ Failed to connect to the database: {}", e);
            return;
        }
    };

    // 3. Create the table automatically if it doesn't exist yet
    let create_table_query = r#"
        CREATE TABLE IF NOT EXISTS route_evaluations (
            id SERIAL PRIMARY KEY,
            route_name VARCHAR(255) NOT NULL,
            estimated_time_hours REAL NOT NULL,
            component_cost REAL NOT NULL,
            dpmo REAL NOT NULL,
            process_yield REAL NOT NULL,
            lean_score REAL NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    "#;

    match sqlx::query(create_table_query).execute(&pool).await {
        Ok(_) => println!("✅ Cloud table 'route_evaluations' is ready."),
        Err(e) => eprintln!("❌ Failed to create table: {}", e),
    }
}