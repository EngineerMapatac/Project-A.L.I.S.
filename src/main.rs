mod six_sigma;
mod engine;
mod ai;
mod report;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Evaluate a JSON file and optionally export to CSV/PDF
    Evaluate {
        #[arg(short, long)]
        data: String,
        #[arg(long)]
        csv: Option<String>,
        #[arg(long)]
        pdf: Option<String>,
    },
    /// Ask A.L.I.S. to suggest a route using AI
    Suggest {
        #[arg(short, long)]
        prompt: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Evaluate { data, csv, pdf } => {
            // ... (Existing JSON reading logic) ...
            let routes = engine::load_from_json(data); 
            engine::evaluate_routes(&routes);

            if let Some(path) = csv {
                engine::export_to_csv(&routes, &path).expect("CSV Export Failed");
            }
            if let Some(path) = pdf {
                report::generate_pdf_report(&routes, &path);
            }
        }
        Commands::Suggest { prompt } => {
            let suggestions = ai::suggest_routes(prompt).await;
            engine::evaluate_routes(&suggestions);
        }
    }
}