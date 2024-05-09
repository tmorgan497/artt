use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use glob::Pattern;

/// Simple program to display a directory tree with exclusions
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The directory to display
    #[arg(default_value = ".")]
    dir: String,

    /// Maximum depth to display
    #[arg(short, long, default_value_t = usize::MAX)]
    depth: usize,

    /// Patterns to exclude (comma-separated)
    #[arg(short = 'E', long)]
    exclude: Option<String>,
}

fn main() {
    let args = Args::parse();
    let exclude_patterns = parse_exclude_patterns(&args.exclude);
    display_tree(&PathBuf::from(&args.dir), args.depth, 0, &exclude_patterns);
}

fn parse_exclude_patterns(patterns: &Option<String>) -> Vec<Pattern> {
    match patterns {
        Some(p) => p.split(',')
                    .filter_map(|pat| Pattern::new(pat).ok())
                    .collect(),
        None => Vec::new(),
    }
}

fn should_exclude(path: &Path, patterns: &[Pattern]) -> bool {
    let mut path_str = path.to_string_lossy().replace("\\", "/"); // Normalize path
    if path_str.starts_with("./") {
        path_str = path_str.replacen("./", "", 1);
    }
    let normalized_path = if path.is_dir() {
        format!("{}/", path_str)
    } else {
        path_str.to_string()
    };

    // dbg!(&path_str, &normalized_path); // Debug output for paths

    patterns.iter().any(|p| {
        // dbg!(p, p.matches(&normalized_path), p.matches(&path_str)); // Debug pattern matching
        p.matches(&normalized_path) || p.matches(&path_str)
    })
}

fn display_tree(path: &Path, max_depth: usize, current_depth: usize, exclude_patterns: &[Pattern]) {
    if current_depth > max_depth {
        return;
    }

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap();
        let full_path = path.to_string_lossy().replace("\\", "/"); // Normalize full path

        if should_exclude(&path, exclude_patterns) {
            continue;
        }

        println!(
            "{}{}",
            "  ".repeat(current_depth),
            name
        );

        if path.is_dir() {
            display_tree(&path, max_depth, current_depth + 1, exclude_patterns);
        }
    }
}
