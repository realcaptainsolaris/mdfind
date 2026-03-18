use chrono::Local;
use clap::{Parser, Subcommand};
use dialoguer::Input;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

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
    let base = dirs::home_dir().unwrap().join("docs/mdfind");

    match cli.command {
        Some(Commands::Create) => create_note(&base),
        None => search_notes(&base, cli.query, cli.tag),
    }
}

fn search_notes(base: &PathBuf, query: Option<String>, tag: Option<String>) {
    let re = Regex::new(r"^(#{1,6})\s+(.*)").unwrap();

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

        for (i, line) in content.lines().enumerate() {
            // tags sammeln
            if line.starts_with("tags:") {
                file_tags = line
                    .replace("tags:", "")
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
            }

            // header finden
            if let Some(cap) = re.captures(line) {
                let title = cap.get(2).unwrap().as_str();

                // filter
                if let Some(ref q) = query {
                    if !title.to_lowercase().contains(&q.to_lowercase()) {
                        continue;
                    }
                }

                if let Some(ref t) = tag {
                    if !file_tags.contains(t) && category.as_ref() != t {
                        continue;
                    }
                }

                println!("{}:{}: {}", path.display(), i + 1, title);
            }
        }
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

    let filename = title.to_lowercase().replace(" ", "_") + ".md";
    path.push(filename);

    let content = format!("# {}\n\ndate: {}\ntags: {}\n\n", title, date, tags);

    fs::write(&path, content).unwrap();

    println!("Created: {}", path.display());
}
