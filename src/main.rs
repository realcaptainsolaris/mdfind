//! mdfind – Simple Markdown Note Search CLI
//!
//! A lightweight command-line tool to create and search Markdown notes
//! organized in a folder structure under `~/docs/mdfind`.
//!
//! Features:
//! - Create new notes with title, tags, and category (folder)
//! - Search notes by header titles (Markdown `#` syntax)
//! - Filter results by tag or category
//! - Recursively scans all `.md` files
//!
//! Note format:
//! Each note is expected to follow a simple structure:
//!
//! ```md
//! # Title
//!
//! date: YYYY-MM-DD
//! tags: tag1, tag2, tag3
//! ```
//!
//! Usage:
//!
//! ```bash
//! mdfind create
//! mdfind <query>
//! mdfind --tag rust
//! mdfind <query> --tag rust
//! ```
//!
//! Search behavior:
//! - Searches only within Markdown headers (`#`, `##`, etc.)
//! - Matches are case-insensitive
//! - Tag filtering uses the `tags:` field or the parent folder name
//!
//! Limitations:
//! - No fuzzy search (simple substring match only)
//! - Tags must be defined in a single `tags:` line
//! - Entire files are loaded into memory
//!
//! Intended use:
//! Personal knowledge base with fast CLI lookup.
//!

use chrono::Local;
use clap::{Parser, Subcommand};
use colored::*;
use dialoguer::Input;
use regex::Regex;
use sanitize_filename::sanitize;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

const DIR_NAME: &str = "docs/mdfind";

#[derive(Parser)]
#[command(name = "mdfind")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Tag filter
    #[arg(short, long)]
    tag: Option<String>,

    /// Search term
    query: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Create,
}

fn main() {
    let cli = Cli::parse();

    let base = match dirs::home_dir() {
        Some(dir) => dir.join(DIR_NAME),
        None => {
            eprintln!("Could not determine home directory");
            return;
        }
    };

    if !base.exists() {
        eprintln!("Base directory does not exist: {}", base.display());
        return;
    }

    match cli.command {
        Some(Commands::Create) => create_note(&base),
        None => search_notes(&base, cli.query, cli.tag),
    }
}
fn search_notes(base: &PathBuf, query: Option<String>, tag: Option<String>) {
    let re = Regex::new(r"^(#{1,6})\s+(.*)").unwrap();
    let mut results = Vec::new();

    for entry in WalkDir::new(base)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|e| e == "md").unwrap_or(false))
    {
        let path = entry.path();
        let content = fs::read_to_string(path).unwrap_or_default();

        let category = path
            .parent()
            .and_then(|p| p.file_name())
            .unwrap_or_default()
            .to_string_lossy();

        let mut file_tags = Vec::new();
        let mut match_result: Option<(usize, String)> = None;

        for (i, line) in content.lines().enumerate() {
            if line.trim_start().starts_with("tags:") {
                file_tags = line
                    .replace("tags:", "")
                    .split(',')
                    .map(|s| s.trim().to_lowercase())
                    .collect();
            }

            if let Some(cap) = re.captures(line) {
                let title = cap.get(2).unwrap().as_str();

                if let Some(ref q) = query {
                    if !title.to_lowercase().contains(&q.to_lowercase()) {
                        continue;
                    }
                }

                if let Some(ref t) = tag {
                    let t = t.to_lowercase();
                    if !file_tags.contains(&t) && category.to_lowercase() != t {
                        continue;
                    }
                }

                if match_result.is_none() {
                    match_result = Some((i + 1, title.to_string()));
                }
            }
        }

        if let Some((line, title)) = match_result {
            results.push(format!("{}:{}: {}", path.display(), line, title));
        }
    }

    results.sort();

    for r in &results {
        println!(
            "{}",
            r.replace(
                ": ",
                &format!(": {}", r.split(": ").last().unwrap().green())
            )
        );
    }
}

fn create_note(base: &PathBuf) {
    let title: String = Input::new().with_prompt("Title").interact_text().unwrap();

    let tags: String = Input::new()
        .with_prompt("Tags (comma separated)")
        .interact_text()
        .unwrap();

    let category: String = Input::new()
        .with_prompt("Category (folder)")
        .interact_text()
        .unwrap();

    let date = Local::now().format("%Y-%m-%d").to_string();

    let mut path = base.join(&category);
    fs::create_dir_all(&path).unwrap();

    let filename = sanitize(title.to_lowercase()) + ".md";
    path.push(filename);

    let content = format!("# {}\n\ndate: {}\ntags: {}\n\n", title, date, tags);

    fs::write(&path, content).unwrap();

    println!("Created: {}", path.display());
}
