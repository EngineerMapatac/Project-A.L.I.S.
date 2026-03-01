// src/engine.rs

#[derive(Debug)]
pub struct RouteOption {
    pub route_name: String,
    pub estimated_time_hours: f64,
    pub component_cost: f64,
    pub total_opportunities: f64, // Potential points of failure (e.g., API calls, manual inputs)
    pub estimated_defects: f64,   // Expected bugs or bottlenecks based on historical data
}

impl RouteOption {
    /// Calculates the DPMO for this specific technical route.
    pub fn calculate_dpmo(&self) -> f64 {
        if self.total_opportunities == 0.0 {
            return 0.0; // Prevent division by zero
        }
        (self.estimated_defects / self.total_opportunities) * 1_000_000.0
    }

    /// Calculates the estimated process yield percentage.
    pub fn calculate_process_yield(&self) -> f64 {
        if self.total_opportunities == 0.0 {
            return 100.0;
        }
        (1.0 - (self.estimated_defects / self.total_opportunities)) * 100.0
    }

    /// Generates a final 'Lean Score' (Lower is better).
    /// Applies weights to time, cost, and projected defects.
    pub fn calculate_lean_score(&self) -> f64 {
        let weight_time = 0.4;
        let weight_cost = 0.3;
        let weight_quality = 0.3;
        
        // Normalize DPMO slightly so it doesn't completely overpower time and cost in the algorithm
        let quality_penalty = self.calculate_dpmo() / 1000.0; 
        
        (self.estimated_time_hours * weight_time) + 
        (self.component_cost * weight_cost) + 
        (quality_penalty * weight_quality)
    }
}