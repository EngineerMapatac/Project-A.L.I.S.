// src/api.rs
use axum::{routing::{get, post}, Json, Router}; // Added 'get' here!
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use crate::engine::RouteOption;

#[derive(Serialize)]
pub struct EvaluatedRoute {
    pub route_name: String,
    pub lean_score: f32,
    pub dpmo: f32,
    pub process_yield: f32,
}

pub async fn start_server() {
    let app = Router::new()
        .route("/", get(root_handler)) // Added the home page route!
        .route("/api/evaluate", post(evaluate_handler))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🌐 A.L.I.S. Web Server starting on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// This is what the browser will see when you click the link
async fn root_handler() -> &'static str {
    "🚀 A.L.I.S. Web Engine is Online and Ready! (Send POST requests to /api/evaluate)"
}

async fn evaluate_handler(Json(payload): Json<Vec<RouteOption>>) -> Json<Vec<EvaluatedRoute>> {
    println!("📥 Received web request to evaluate {} routes", payload.len());
    
    let mut results = Vec::new();
    
    for route in payload {
        let dpmo = crate::six_sigma::calculate_dpmo(route.estimated_defects, route.total_opportunities);
        let process_yield = crate::six_sigma::calculate_yield(route.estimated_defects, route.total_opportunities);
        let lean_score = route.calculate_lean_score();

        results.push(EvaluatedRoute {
            route_name: route.route_name,
            lean_score: lean_score as f32,
            dpmo: dpmo as f32,
            process_yield: process_yield as f32,
        });
    }

    Json(results)
}