mod ai;
mod engine;
mod report;
mod six_sigma;

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
        #[arg(long)]
        pdf: Option<String>,
        // The new flag for saving to the Render PostgreSQL database
        #[arg(long)]
        save: bool, 
    },
    Suggest {
        #[arg(short, long)]
        prompt: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // This is the line that was missing!
    match &cli.command {
        Commands::Evaluate { data, csv, pdf, save } => {
            let raw_json = fs::read_to_string(data).expect("Unable to read file");
            let routes: Vec<engine::RouteOption> = serde_json::from_str(&raw_json).expect("JSON error");
            
            engine::evaluate_routes(&routes);

            if let Some(path) = csv {
                engine::export_to_csv(&routes, path).expect("CSV export failed");
            }

            if let Some(path) = pdf {
                report::generate_pdf_report(&routes, path);
            }

            if *save {
                println!("☁️ Connecting to Render Database...");
                // The sqlx logic will go here next!
            }
        }
        Commands::Suggest { prompt } => {
            let suggestions = ai::suggest_routes(prompt).await;
            engine::evaluate_routes(&suggestions);
        }
    }
}