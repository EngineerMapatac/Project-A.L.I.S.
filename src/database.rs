use sqlx::postgres::PgPoolOptions;
use crate::engine::RouteOption;

pub async fn save_to_render(route: &RouteOption, db_url: &str) {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url).await.unwrap();

    let dpmo = crate::six_sigma::calculate_dpmo(route.estimated_defects, route.total_opportunities);
    let lean_score = route.calculate_lean_score();

    // Push the data to your Render PostgreSQL database
    sqlx::query!(
        "INSERT INTO evaluations (route_name, time_hours, cost, dpmo, lean_score) VALUES ($1, $2, $3, $4, $5)",
        route.route_name, route.estimated_time_hours, route.component_cost, dpmo, lean_score
    )
    .execute(&pool)
    .await
    .unwrap();

    println!("☁️ Route '{}' successfully saved to Render Cloud Database.", route.route_name);
}