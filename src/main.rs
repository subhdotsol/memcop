mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "memcop", about = "Rust systems inspection CLI", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Inspect the size, alignment, and layout of supported Rust types
    Layout {
        /// Type to inspect (optional; if omitted, show all supported types)
        #[arg(long)]
        r#type: Option<String>,
    },
    /// Run experiments with stack values, boxes, vectors, and strings
    Alloc,
    /// Inspect the current process's Linux virtual memory mappings
    Maps,
    /// Read memory-related metrics from Linux /proc process information
    Status,
    /// Combine layout, allocation, and process-memory observations into one report
    Report,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Layout { r#type } => commands::layout::run(r#type.as_deref()),
        Commands::Alloc => commands::alloc::run(),
        Commands::Maps => commands::maps::run(),
        Commands::Status => commands::status::run(),
        Commands::Report => commands::report::run(),
    }
}
