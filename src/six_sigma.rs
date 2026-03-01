// src/six_sigma.rs

/// Calculates Defects Per Million Opportunities (DPMO).
/// Formula: (Defects / Opportunities) * 1,000,000
pub fn calculate_dpmo(defects: f64, opportunities: f64) -> f64 {
    if opportunities == 0.0 {
        return 0.0; // Prevent division by zero
    }
    (defects / opportunities) * 1_000_000.0
}

/// Calculates the Process Yield as a percentage.
/// Formula: (1 - (Defects / Opportunities)) * 100
pub fn calculate_yield(defects: f64, opportunities: f64) -> f64 {
    if opportunities == 0.0 {
        return 100.0; // If there are no opportunities for defects, yield is 100%
    }
    (1.0 - (defects / opportunities)) * 100.0
}