mod six_sigma; // Declares the new module
use clap::{Parser, Subcommand};

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
            println!("📂 Loading dataset from: {}", data);
            println!("✅ Evaluation complete. (Logic pending in engine.rs)");
        }
        Commands::Analyze { defects, opportunities } => {
            println!("📊 Running Lean Six Sigma analysis...\n");
            
            // Call the functions from our new module
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