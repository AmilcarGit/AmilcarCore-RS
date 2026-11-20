use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
    /// Filesystem operations.
    File {
        #[command(subcommand)]
        command: FileCommands,
    },
}

#[derive(Debug, Subcommand)]
enum FileCommands {
    /// Inspect a file or directory without reading its contents.
    Inspect { path: PathBuf },
    /// Compare two files and report the first differing byte.
    Diff { left: PathBuf, right: PathBuf },
    /// Find files recursively by extension and/or filename substring.
    Find {
        path: PathBuf,
        #[arg(long)]
        extension: Option<String>,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, default_value_t = 100_000)]
        max_entries: usize,
    },
    /// Copy a regular file. Refuses to overwrite unless --overwrite is supplied.
    Copy {
        source: PathBuf,
        destination: PathBuf,
        #[arg(long)]
        overwrite: bool,
    },
    /// Move a file or directory to a destination that must not exist.
    Move { source: PathBuf, destination: PathBuf },
    /// Rename a file or directory within its current parent directory.
    Rename { path: PathBuf, new_name: String },
    /// Create a directory and any missing parent directories.
    Mkdir { path: PathBuf },
    /// Remove one regular file. This does not remove directories.
    Remove { path: PathBuf },
    /// Remove an empty directory only.
    Rmdir { path: PathBuf },
    /// Calculate the SHA-256 digest of a regular file.
    Hash { path: PathBuf },
    /// List groups of files with identical SHA-256 digests.
    Duplicates {
        path: PathBuf,
        #[arg(long, default_value_t = 100_000)]
        max_entries: usize,
    },
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command.unwrap_or(Commands::Info) {
        Commands::Info => {
            let engine = amilcar_core::Engine::new();
            println!("AmilcarCore-RS {}", env!("CARGO_PKG_VERSION"));
            println!("Maximum inspected file size: {} bytes", engine.config().max_file_size);
            println!("Execution mode: local");
        }
        Commands::File { command } => match command {
            FileCommands::Inspect { path } => {
                let result = amilcar_core::Engine::new().inspect_path(&path)?;
                println!("Path: {}", result.path.display());
                println!("Type: {}", if result.is_directory { "directory" } else if result.is_file { "file" } else { "other" });
                println!("Size: {} bytes", result.size);
            }
            FileCommands::Diff { left, right } => {
                let result = amilcar_fs::compare_files(&left, &right)?;
                if result.identical {
                    println!("Files are identical ({} bytes).", result.left_size);
                } else {
                    println!("Files differ at byte {}.", result.first_difference.unwrap_or(0));
                    println!("Left: {} bytes; right: {} bytes.", result.left_size, result.right_size);
                    std::process::exit(2);
                }
            }
            FileCommands::Find { path, extension, name, max_entries } => {
                let filter = amilcar_fs::FileFilter { extension, name_contains: name };
                let options = amilcar_fs::DirectoryOptions { max_entries };
                for found in amilcar_fs::find_files(path, &filter, &options)? {
                    println!("{}", found.display());
                }
            }
            FileCommands::Copy { source, destination, overwrite } => {
                let bytes = amilcar_fs::copy_file(source, destination, overwrite)?;
                println!("Copied {bytes} bytes.");
            }
            FileCommands::Move { source, destination } => {
                amilcar_fs::move_path(source, destination)?;
                println!("Path moved successfully.");
            }
            FileCommands::Rename { path, new_name } => {
                amilcar_fs::rename_path(path, new_name)?;
                println!("Path renamed successfully.");
            }
            FileCommands::Mkdir { path } => {
                amilcar_fs::create_directory(path)?;
                println!("Directory created.");
            }
            FileCommands::Remove { path } => {
                amilcar_fs::remove_file(path)?;
                println!("File removed.");
            }
            FileCommands::Rmdir { path } => {
                amilcar_fs::remove_empty_directory(path)?;
                println!("Empty directory removed.");
            }
            FileCommands::Hash { path } => {
                println!("{}", amilcar_fs::sha256_file(path)?);
            }
            FileCommands::Duplicates { path, max_entries } => {
                let groups = amilcar_fs::find_duplicates(path, max_entries)?;
                if groups.is_empty() {
                    println!("No duplicate files found.");
                } else {
                    for group in groups {
                        println!("SHA-256: {}", group.sha256);
                        for file in group.files {
                            println!("  {}", file.display());
                        }
                    }
                }
            }
        },
    }
    Ok(())
}
