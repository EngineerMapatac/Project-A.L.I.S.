// src/database.rs
use crate::engine::RouteOption;
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenvy::dotenv;

pub async fn save_routes(routes: &[RouteOption]) {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("❌ DATABASE_URL must be set in .env");

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("❌ Failed to connect to Render: {}", e);
            return;
        }
    };

    // 1. Create the table if it doesn't exist
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
    let _ = sqlx::query(create_table_query).execute(&pool).await;
    println!("✅ Connected to cloud database! Saving routes...");

    // 2. Loop through the routes and save them to PostgreSQL
    for route in routes {
        let dpmo = crate::six_sigma::calculate_dpmo(route.estimated_defects, route.total_opportunities);
        let yield_pct = crate::six_sigma::calculate_yield(route.estimated_defects, route.total_opportunities);
        let score = route.calculate_lean_score();

        match sqlx::query!(
            "INSERT INTO route_evaluations (route_name, estimated_time_hours, component_cost, dpmo, process_yield, lean_score) VALUES ($1, $2, $3, $4, $5, $6)",
            route.route_name,
            route.estimated_time_hours as f32,
            route.component_cost as f32,
            dpmo as f32,
            yield_pct as f32,
            score as f32
        )
        .execute(&pool)
        .await
        {
            Ok(_) => println!("  ☁️ Saved: {}", route.route_name),
            Err(e) => eprintln!("  ❌ Failed to save {}: {}", route.route_name, e),
        }
    }
    
    println!("🚀 All data successfully synced to Render PostgreSQL!");
}