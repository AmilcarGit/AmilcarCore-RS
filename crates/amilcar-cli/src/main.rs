use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "amilcar", version, about = "Local native processing engine")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Display engine configuration and build information.
    Info,
}

fn main() {
    let cli = Cli::parse();
    match cli.command.unwrap_or(Commands::Info) {
        Commands::Info => {
            let engine = amilcar_core::Engine::new();
            println!("AmilcarCore-RS {}", env!("CARGO_PKG_VERSION"));
            println!(
                "Maximum inspected file size: {} bytes",
                engine.config().max_file_size
            );
            println!("Execution mode: local");
        }
    }
}
