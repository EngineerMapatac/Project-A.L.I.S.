// src/engine.rs
use serde::Deserialize;
use crate::six_sigma; // Pull in our math module

// The Deserialize trait allows serde to automatically parse JSON into this struct
#[derive(Debug, Deserialize)]
pub struct RouteOption {
    pub route_name: String,
    pub estimated_time_hours: f64,
    pub component_cost: f64,
    pub total_opportunities: f64,
    pub estimated_defects: f64,
}

impl RouteOption {
    /// Generates a final 'Lean Score' (Lower is better).
    pub fn calculate_lean_score(&self) -> f64 {
        let dpmo = six_sigma::calculate_dpmo(self.estimated_defects, self.total_opportunities);
        
        // Strategic weights: Time (40%), Cost (30%), Quality/Defects (30%)
        let weight_time = 0.4;
        let weight_cost = 0.3;
        let weight_quality = 0.3;
        
        let quality_penalty = dpmo / 1000.0; 
        
        (self.estimated_time_hours * weight_time) + 
        (self.component_cost * weight_cost) + 
        (quality_penalty * weight_quality)
    }
}

/// Evaluates and prints the metrics for a list of routes
pub fn evaluate_routes(routes: &[RouteOption]) {
    println!("Evaluating {} potential engineering routes...\n", routes.len());
    
    for route in routes {
        let dpmo = six_sigma::calculate_dpmo(route.estimated_defects, route.total_opportunities);
        let yield_pct = six_sigma::calculate_yield(route.estimated_defects, route.total_opportunities);
        let score = route.calculate_lean_score();

        println!("🛠️ Route: {}", route.route_name);
        println!("  ├─ DPMO: {:.2}", dpmo);
        println!("  ├─ Process Yield: {:.2}%", yield_pct);
        println!("  └─ Lean Score: {:.2} (Lower is better)\n", score);
    }
}

 // src/engine.rs additions
use std::error::Error;
use csv::Writer;

pub fn export_to_csv(routes: &[RouteOption], filename: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(filename)?;

    // Write the header row
    wtr.write_record(&["Route Name", "Time (Hrs)", "Cost", "DPMO", "Yield (%)", "Lean Score"])?;

    for route in routes {
        // We reuse your six_sigma logic for the export values
        let dpmo = crate::six_sigma::calculate_dpmo(route.estimated_defects, route.total_opportunities);
        let yield_pct = crate::six_sigma::calculate_yield(route.estimated_defects, route.total_opportunities);
        let score = route.calculate_lean_score();

        wtr.serialize((
            &route.route_name,
            route.estimated_time_hours,
            route.component_cost,
            format!("{:.2}", dpmo),
            format!("{:.2}%", yield_pct),
            format!("{:.2}", score),
        ))?;
    }

    wtr.flush()?;
    println!("✅ CSV Report successfully exported to: {}", filename);
    Ok(())
}