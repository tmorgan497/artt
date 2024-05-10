use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use glob::Pattern;
use std::env;

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

    /// Include hidden files
    #[arg(short = 'a', long)]
    all: bool,

    /// Use Nerd Fonts icons
    #[arg(short = 'b', long)]
    nerd_fonts: bool,
}

fn main() {
    let args = Args::parse();
    let exclude_patterns = parse_exclude_patterns(&args.exclude);
    let mut file_count = 0;
    let mut dir_count = 0;

    let nerd_fonts_supported = args.nerd_fonts || env::var("NERDFONTS").is_ok();

    println!(".");  // Start character

    display_tree(
        &PathBuf::from(&args.dir),
        args.depth,
        0,
        &exclude_patterns,
        "",
        &mut file_count,
        &mut dir_count,
        args.all,
        nerd_fonts_supported
    );

    println!("\n{} directories, {} files", dir_count, file_count);
}

fn parse_exclude_patterns(patterns: &Option<String>) -> Vec<Pattern> {
    match patterns {
        Some(p) => p.split(',')
                    .filter_map(|pat| Pattern::new(pat).ok())
                    .collect(),
        None => Vec::new(),
    }
}

fn should_exclude(path: &Path, patterns: &[Pattern], include_hidden: bool) -> bool {
    let mut path_str = path.to_string_lossy().replace("\\", "/"); // Normalize path
    if path_str.starts_with("./") {
        path_str = path_str.replacen("./", "", 1);
    }
    let normalized_path = if path.is_dir() {
        format!("{}/", path_str)
    } else {
        path_str.to_string()
    };

    let is_hidden = path.file_name()
                        .and_then(|name| name.to_str())
                        .map(|name| name.starts_with('.'))
                        .unwrap_or(false);

    (!include_hidden && is_hidden) || patterns.iter().any(|p| p.matches(&normalized_path) || p.matches(&path_str))
}

fn display_tree(
    path: &Path,
    max_depth: usize,
    current_depth: usize,
    exclude_patterns: &[Pattern],
    prefix: &str,
    file_count: &mut usize,
    dir_count: &mut usize,
    include_hidden: bool,
    nerd_fonts_supported: bool
) {
    if current_depth > max_depth {
        return;
    }

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries.collect::<Result<Vec<_>, _>>().unwrap_or_default(),
        Err(_) => return,
    };

    let mut non_excluded_entries = vec![];

    for entry in entries {
        let entry_path = entry.path();
        if !should_exclude(&entry_path, exclude_patterns, include_hidden) {
            non_excluded_entries.push(entry);
        }
    }

    for (index, entry) in non_excluded_entries.iter().enumerate() {
        let entry = entry.path();
        let name = entry.file_name().unwrap().to_str().unwrap();
        let is_last = index == non_excluded_entries.len() - 1;

        let icon = if entry.is_dir() {
            if nerd_fonts_supported { " " } else { "[DIR] " }
        } else {
            if nerd_fonts_supported { " " } else { "[FILE] " }
        };

        println!(
            "{}{}{}{}",
            prefix,
            if is_last { "└── " } else { "├── " },
            icon,
            name
        );

        if entry.is_dir() {
            *dir_count += 1;
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            display_tree(
                &entry,
                max_depth,
                current_depth + 1,
                exclude_patterns,
                &new_prefix,
                file_count,
                dir_count,
                include_hidden,
                nerd_fonts_supported
            );
        } else {
            *file_count += 1;
        }
    }
}
