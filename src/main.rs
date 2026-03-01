mod six_sigma;
mod engine; // Declare the new engine module

use clap::{Parser, Subcommand};
use std::fs; // Used to read the file

// ... (Keep your existing Cli and Commands structs the same) ...

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Evaluate { data } => {
            println!("⚙️  A.L.I.S. Engine Initializing...");
            println!("📂 Loading dataset from: {}\n", data);
            
            // 1. Read the raw text from the file
            let raw_json = match fs::read_to_string(data) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("❌ Failed to read file: {}", e);
                    return;
                }
            };

            // 2. Parse the JSON into a Vector of RouteOption structs
            let routes: Vec<engine::RouteOption> = match serde_json::from_str(&raw_json) {
                Ok(parsed) => parsed,
                Err(e) => {
                    eprintln!("❌ Failed to parse JSON: {}", e);
                    return;
                }
            };

            // 3. Run the evaluation
            engine::evaluate_routes(&routes);
        }
        Commands::Analyze { defects, opportunities } => {
            // ... (Keep your existing Analyze logic the same) ...
            println!("📊 Running Lean Six Sigma analysis...\n");
            let dpmo = six_sigma::calculate_dpmo(*defects, *opportunities);
            let process_yield = six_sigma::calculate_yield(*defects, *opportunities);

            println!("Defects Logged: {}", defects);
            println!("Total Opportunities: {}", opportunities);
            println!("-----------------------------------");
            println!("DPMO: {:.2}", dpmo);
            println!("Process Yield: {:.2}%", process_yield);
        }
    }
}