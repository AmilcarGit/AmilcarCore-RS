use clap::{Parser, Subcommand};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

const MAX_TEXT_BYTES: u64 = 64 * 1024 * 1024;

#[derive(Debug, Parser)]
#[command(
    name = "amilcar",
    version,
    about = "Local native processing engine",
    long_about = "AmilcarCore-RS: local file, text and structured-data processing."
)]
struct Cli {
    /// Print successful command results as JSON for scripts and automation.
    #[arg(long, global = true)]
    json: bool,
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
    /// UTF-8 text processing.
    Text {
        #[command(subcommand)]
        command: TextCommands,
    },
    /// JSON validation and formatting.
    Json {
        #[command(subcommand)]
        command: JsonCommands,
    },
    /// CSV validation, inspection and transformation.
    Csv {
        #[command(subcommand)]
        command: CsvCommands,
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
    Move {
        source: PathBuf,
        destination: PathBuf,
    },
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

#[derive(Debug, Subcommand)]
enum TextCommands {
    /// Count bytes, Unicode characters, words and lines.
    Stats { path: PathBuf },
    /// Normalize CRLF and CR line endings to LF.
    Normalize {
        source: PathBuf,
        destination: PathBuf,
        #[arg(long)]
        overwrite: bool,
    },
}

#[derive(Debug, Subcommand)]
enum JsonCommands {
    /// Validate a JSON file.
    Validate { path: PathBuf },
    /// Format a JSON file into a destination file.
    Format {
        source: PathBuf,
        destination: PathBuf,
        /// Produce compact JSON instead of pretty-printed JSON.
        #[arg(long)]
        compact: bool,
        /// Allow replacing an existing destination.
        #[arg(long)]
        overwrite: bool,
    },
}

#[derive(Debug, Subcommand)]
enum CsvCommands {
    /// Validate a CSV file and report its structure.
    Validate { path: PathBuf },
    /// Inspect CSV headers, columns and record count.
    Inspect { path: PathBuf },
    /// Rewrite a CSV file with consistent quoting and record separators.
    Format {
        source: PathBuf,
        destination: PathBuf,
        #[arg(long)]
        overwrite: bool,
    },
    /// Keep only the requested exact header names, in the given order.
    Select {
        source: PathBuf,
        destination: PathBuf,
        #[arg(long, value_delimiter = ',', required = true)]
        columns: Vec<String>,
        #[arg(long)]
        overwrite: bool,
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
    let json_output = cli.json;

    match cli.command.unwrap_or(Commands::Info) {
        Commands::Info => {
            let engine = amilcar_core::Engine::new();
            let version = env!("CARGO_PKG_VERSION");
            let max_file_size = engine.config().max_file_size;
            emit(
                json_output,
                format!(
                    "AmilcarCore-RS {version}\nMaximum inspected file size: {max_file_size} bytes\nExecution mode: local"
                ),
                json!({
                    "name": "AmilcarCore-RS",
                    "version": version,
                    "max_file_size_bytes": max_file_size,
                    "execution_mode": "local"
                }),
            );
        }
        Commands::File { command } => match command {
            FileCommands::Inspect { path } => {
                let result = amilcar_core::Engine::new().inspect_path(&path)?;
                let kind = if result.is_directory {
                    "directory"
                } else if result.is_file {
                    "file"
                } else {
                    "other"
                };
                emit(
                    json_output,
                    format!(
                        "Path: {}\nType: {kind}\nSize: {} bytes",
                        result.path.display(),
                        result.size
                    ),
                    json!({
                        "path": result.path,
                        "type": kind,
                        "size_bytes": result.size
                    }),
                );
            }
            FileCommands::Diff { left, right } => {
                let result = amilcar_fs::compare_files(&left, &right)?;
                let payload = json!({
                    "identical": result.identical,
                    "first_difference": result.first_difference,
                    "left_size_bytes": result.left_size,
                    "right_size_bytes": result.right_size
                });
                if result.identical {
                    emit(
                        json_output,
                        format!("Files are identical ({} bytes).", result.left_size),
                        payload,
                    );
                } else {
                    emit(
                        json_output,
                        format!(
                            "Files differ at byte {}.\nLeft: {} bytes; right: {} bytes.",
                            result.first_difference.unwrap_or(0),
                            result.left_size,
                            result.right_size
                        ),
                        payload,
                    );
                    std::process::exit(2);
                }
            }
            FileCommands::Find {
                path,
                extension,
                name,
                max_entries,
            } => {
                let filter = amilcar_fs::FileFilter {
                    extension,
                    name_contains: name,
                };
                let options = amilcar_fs::DirectoryOptions { max_entries };
                let found = amilcar_fs::find_files(path, &filter, &options)?;
                let paths = found
                    .iter()
                    .map(|entry| entry.display().to_string())
                    .collect::<Vec<_>>();
                if json_output {
                    println!("{}", serde_json::to_string(&paths)?);
                } else {
                    for path in paths {
                        println!("{path}");
                    }
                }
            }
            FileCommands::Copy {
                source,
                destination,
                overwrite,
            } => {
                let bytes = amilcar_fs::copy_file(&source, &destination, overwrite)?;
                emit(
                    json_output,
                    format!("Copied {bytes} bytes."),
                    json!({
                        "success": true,
                        "bytes_copied": bytes,
                        "destination": destination
                    }),
                );
            }
            FileCommands::Move {
                source,
                destination,
            } => {
                amilcar_fs::move_path(&source, &destination)?;
                emit(
                    json_output,
                    "Path moved successfully.".to_owned(),
                    json!({"success": true, "operation": "move", "destination": destination}),
                );
            }
            FileCommands::Rename { path, new_name } => {
                amilcar_fs::rename_path(&path, &new_name)?;
                emit(
                    json_output,
                    "Path renamed successfully.".to_owned(),
                    json!({"success": true, "operation": "rename", "new_name": new_name}),
                );
            }
            FileCommands::Mkdir { path } => {
                amilcar_fs::create_directory(&path)?;
                emit(
                    json_output,
                    "Directory created.".to_owned(),
                    json!({"success": true, "path": path}),
                );
            }
            FileCommands::Remove { path } => {
                amilcar_fs::remove_file(&path)?;
                emit(
                    json_output,
                    "File removed.".to_owned(),
                    json!({"success": true, "removed": path}),
                );
            }
            FileCommands::Rmdir { path } => {
                amilcar_fs::remove_empty_directory(&path)?;
                emit(
                    json_output,
                    "Empty directory removed.".to_owned(),
                    json!({"success": true, "removed": path}),
                );
            }
            FileCommands::Hash { path } => {
                let digest = amilcar_fs::sha256_file(&path)?;
                emit(
                    json_output,
                    digest.clone(),
                    json!({"algorithm": "SHA-256", "digest": digest, "path": path}),
                );
            }
            FileCommands::Duplicates { path, max_entries } => {
                let groups = amilcar_fs::find_duplicates(path, max_entries)?;
                let payload = groups
                    .iter()
                    .map(|group| {
                        json!({
                            "sha256": group.sha256,
                            "files": group.files.iter().map(|file| file.display().to_string()).collect::<Vec<_>>()
                        })
                    })
                    .collect::<Vec<_>>();
                if json_output {
                    println!("{}", serde_json::to_string(&payload)?);
                } else if groups.is_empty() {
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
        Commands::Text { command } => match command {
            TextCommands::Stats { path } => {
                let input = read_text(&path)?;
                let result = amilcar_data::text::stats(&input);
                emit(
                    json_output,
                    format!(
                        "Bytes: {}\nCharacters: {}\nWords: {}\nLines: {}",
                        result.bytes, result.characters, result.words, result.lines
                    ),
                    json!({
                        "path": path,
                        "bytes": result.bytes,
                        "characters": result.characters,
                        "words": result.words,
                        "lines": result.lines
                    }),
                );
            }
            TextCommands::Normalize {
                source,
                destination,
                overwrite,
            } => {
                let input = read_text(&source)?;
                let normalized = amilcar_data::text::normalize_newlines(&input);
                write_text(&destination, &normalized, overwrite)?;
                emit(
                    json_output,
                    format!("Normalized text written to {}.", destination.display()),
                    json!({"success": true, "destination": destination}),
                );
            }
        },
        Commands::Json { command } => match command {
            JsonCommands::Validate { path } => {
                let input = read_text(&path)?;
                amilcar_data::json::validate(&input)?;
                emit(
                    json_output,
                    format!("Valid JSON: {}", path.display()),
                    json!({"valid": true, "path": path}),
                );
            }
            JsonCommands::Format {
                source,
                destination,
                compact,
                overwrite,
            } => {
                let input = read_text(&source)?;
                let formatted = if compact {
                    amilcar_data::json::compact(&input)?
                } else {
                    amilcar_data::json::format(&input)?
                };
                write_text(&destination, &formatted, overwrite)?;
                emit(
                    json_output,
                    format!("Formatted JSON written to {}.", destination.display()),
                    json!({"success": true, "compact": compact, "destination": destination}),
                );
            }
        },
        Commands::Csv { command } => match command {
            CsvCommands::Validate { path } | CsvCommands::Inspect { path } => {
                let input = read_text(&path)?;
                let result = amilcar_data::csv::inspect(&input)?;
                emit(
                    json_output,
                    format!(
                        "Valid CSV: {}\nColumns: {}\nRows: {}\nHeaders: {}",
                        path.display(),
                        result.columns,
                        result.rows,
                        result.headers.join(", ")
                    ),
                    json!({
                        "valid": true,
                        "path": path,
                        "columns": result.columns,
                        "rows": result.rows,
                        "headers": result.headers
                    }),
                );
            }
            CsvCommands::Format {
                source,
                destination,
                overwrite,
            } => {
                let input = read_text(&source)?;
                let formatted = amilcar_data::csv::format(&input)?;
                write_text(&destination, &formatted, overwrite)?;
                emit(
                    json_output,
                    format!("Formatted CSV written to {}.", destination.display()),
                    json!({"success": true, "destination": destination}),
                );
            }
            CsvCommands::Select {
                source,
                destination,
                columns,
                overwrite,
            } => {
                let input = read_text(&source)?;
                let selected = amilcar_data::csv::select_columns(&input, &columns)?;
                write_text(&destination, &selected, overwrite)?;
                emit(
                    json_output,
                    format!("Selected CSV columns written to {}.", destination.display()),
                    json!({"success": true, "columns": columns, "destination": destination}),
                );
            }
        },
    }

    Ok(())
}

fn emit(json_output: bool, human: String, payload: Value) {
    if json_output {
        println!("{payload}");
    } else {
        println!("{human}");
    }
}

fn read_text(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    Ok(amilcar_fs::read_to_string_limited(path, MAX_TEXT_BYTES)?)
}

fn write_text(
    path: &Path,
    contents: &str,
    overwrite: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mode = if overwrite {
        amilcar_fs::WriteMode::Overwrite
    } else {
        amilcar_fs::WriteMode::CreateNew
    };
    amilcar_fs::write_bytes(path, contents.as_bytes(), mode)?;
    Ok(())
}
