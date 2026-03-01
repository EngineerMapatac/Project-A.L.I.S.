mod six_sigma;
mod engine; 

use clap::{Parser, Subcommand};
use std::fs; 

#[derive(Parser)]
#[command(name = "A.L.I.S.")]
#[command(author = "John Mapatac")]
#[command(version = "0.1.0")]
#[command(about = "Optimizes engineering routes using Six Sigma metrics", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Evaluate {
        #[arg(short, long)]
        data: String,
    },
    Analyze {
        #[arg(short, long)]
        defects: f64,
        #[arg(short, long)]
        opportunities: f64,
    }
}

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