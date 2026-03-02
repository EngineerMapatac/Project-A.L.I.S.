mod six_sigma;
mod engine;
mod ai;

use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser)]
#[command(name = "A.L.I.S. v2")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Evaluate {
        #[arg(short, long)]
        data: String,
        #[arg(long)]
        csv: Option<String>,
    },
    Suggest {
        #[arg(short, long)]
        prompt: String,
    },
}

#[tokio::main] // This handles the async AI logic
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Evaluate { data, csv } => {
            let raw_json = fs::read_to_string(data).expect("Unable to read file");
            let routes: Vec<engine::RouteOption> = serde_json::from_str(&raw_json).expect("JSON error");
            
            engine::evaluate_routes(&routes);

            if let Some(path) = csv {
                engine::export_to_csv(&routes, path).expect("CSV export failed");
            }
        }
        Commands::Suggest { prompt } => {
            let suggestions = ai::suggest_routes(prompt).await;
            engine::evaluate_routes(&suggestions);
        }
    }
}

// Inside main.rs match arm
Commands::Evaluate { data, csv, pdf } => {
    let raw_json = fs::read_to_string(data).expect("Unable to read file");
    let routes: Vec<engine::RouteOption> = serde_json::from_str(&raw_json).expect("JSON error");
    
    engine::evaluate_routes(&routes);

    if let Some(path) = csv {
        engine::export_to_csv(&routes, path).expect("CSV export failed");
    }
    
    if let Some(path) = pdf {
        report::generate_pdf_report(&routes, path); // Now it's linked!
    }
}