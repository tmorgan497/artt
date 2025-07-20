/* src\main.rs
Command-line utility to display a directory tree with exclusions
*/

use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The directory to display
    #[arg(default_value = ".")]
    dir: String,

    /// Show all files
    #[arg(short = 'a', long)]
    all: bool,

    /// Use Nerd Fonts icons instead of [DIR] and [FILE]
    #[arg(short = 'b', long, default_value_t = false)]
    nerd_fonts: bool,

    /// Enable color output
    #[arg(short = 'C', long, default_value_t = false)]
    color: bool,

    /// List directories only
    #[arg(short = 'd', long, default_value_t = false)]
    dironly: bool,

    /// Maximum depth to display
    #[arg(short = 'L', long, default_value_t = usize::MAX)]
    depth: usize,

    /// Patterns to ignore (pipe-separated)
    #[arg(short = 'I', long)]
    ignore: Option<String>,

    /// Turn off file/dir count at the end of the tree listing
    #[arg(long, default_value_t = false)]
    noreport: bool,

    /// Debug/Developer mode
    #[arg(long)]
    debug: bool,

    /// Include files that are in .gitignore (default is to ignore them)
    #[arg(long)]
    include_gitignore: bool,
}

fn main() {
    let args = Args::parse();
    let mut file_count = 0;
    let mut dir_count = 0;

    if args.debug {
        println!("Directory: {}", args.dir);
        println!("All: {}", args.all);
        println!("Nerd Fonts: {}", args.nerd_fonts);

        if args.nerd_fonts {
            println!("Directory icon:  ");
            println!("File icon:  ");
        }

        println!("Color: {}", args.color);
        println!("Dironly: {}", args.dironly);
        println!("Depth: {}", args.depth);

        println!("Ignore (input): {:?}", args.ignore);
        let exclude_patterns = parse_exclude_patterns(&args.ignore);
        println!("Ignore (parsed): {:?}", exclude_patterns);
        
        println!("Include gitignore: {}", args.include_gitignore);
        let gitignore_patterns = parse_gitignore(Path::new(&args.dir));
        println!("Gitignore patterns: {:?}", gitignore_patterns);

        println!("No-report: {}", args.noreport);
    }

    println!(".");
    let exclude_patterns = parse_exclude_patterns(&args.ignore);
    let gitignore_patterns = parse_gitignore(Path::new(&args.dir));
    display_tree(
        Path::new(&args.dir),
        0,
        &exclude_patterns,
        &gitignore_patterns,
        "",
        &mut file_count,
        &mut dir_count,
        &args
    );

    if !args.noreport {
        println!("\n{} directories, {} files", dir_count, file_count);
    }
}

fn parse_exclude_patterns(patterns: &Option<String>) -> Vec<String> {
    match patterns {
        Some(p) => p.split('|')
                    .map(|pat| pat.to_string())
                    .collect(),
        None => Vec::new(),
    }
}

fn parse_gitignore(dir: &Path) -> Vec<String> {
    let mut gitignore_patterns = Vec::new();
    let gitignore_path = dir.join(".gitignore");
    
    if gitignore_path.exists() {
        if let Ok(content) = fs::read_to_string(&gitignore_path) {
            for line in content.lines() {
                let line = line.trim();
                // Skip empty lines and comments
                if !line.is_empty() && !line.starts_with('#') {
                    gitignore_patterns.push(line.to_string());
                }
            }
        }
    }
    
    gitignore_patterns
}

fn matches_gitignore_pattern(path: &Path, pattern: &str) -> bool {
    let path_str = path.to_string_lossy().replace("\\", "/");
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();
    
    // Handle different gitignore pattern types
    if pattern.starts_with('/') {
        // Pattern starting with '/' matches from root
        let pattern = &pattern[1..];
        if pattern.ends_with('/') {
            // Directory pattern like "/target/"
            let pattern_name = &pattern[..pattern.len()-1];
            // For root-level directory patterns, just check the filename
            path.is_dir() && file_name == pattern_name
        } else {
            // File pattern like "/Cargo.lock"
            file_name == pattern
        }
    } else if pattern.ends_with('/') {
        // Directory-only pattern
        let pattern_name = &pattern[..pattern.len()-1];
        path.is_dir() && (file_name == pattern_name || path_str.contains(&format!("/{}", pattern_name)))
    } else if pattern.contains('/') {
        // Pattern with path separator
        path_str.contains(pattern)
    } else {
        // Simple pattern - matches file/directory name anywhere
        file_name == pattern || path_str.contains(&format!("/{}", pattern)) || 
        (pattern.contains('*') && simple_glob_match(&file_name, pattern))
    }
}

fn simple_glob_match(text: &str, pattern: &str) -> bool {
    // Very basic glob matching for ** and * patterns
    if pattern == "**" {
        return true;
    }
    
    if pattern.contains("**") {
        // Handle ** pattern (match any number of directories)
        let parts: Vec<&str> = pattern.split("**").collect();
        if parts.len() == 2 {
            let prefix = parts[0];
            let suffix = parts[1];
            return text.starts_with(prefix) && text.ends_with(suffix);
        }
    }
    
    // Simple * matching
    if pattern.contains('*') {
        let parts: Vec<&str> = pattern.split('*').collect();
        if parts.len() == 2 {
            return text.starts_with(parts[0]) && text.ends_with(parts[1]);
        }
    }
    
    text == pattern
}

fn should_exclude(path: &Path, exclude_patterns: &[String], gitignore_patterns: &[String], include_hidden: bool, include_gitignore: bool) -> bool {
    let path_str = path.to_string_lossy().replace("\\", "/"); // Normalize path

    if !include_hidden && path.file_name().unwrap().to_str().unwrap().starts_with('.') {
        return true;
    }

    // Check gitignore patterns first (unless include_gitignore is true)
    if !include_gitignore && !gitignore_patterns.is_empty() {
        for pattern in gitignore_patterns {
            if matches_gitignore_pattern(path, pattern) {
                return true;
            }
        }
    }

    if exclude_patterns.is_empty() {
        return false;
    }

    let normalized_path = if path.is_dir() {
        format!("{}/", path_str)
    } else {
        path_str.to_string()
    };

    exclude_patterns.iter().any(|p| normalized_path.contains(p) || path_str.contains(p))
}

fn display_tree(
    path: &Path,
    current_depth: usize,
    exclude_patterns: &[String],
    gitignore_patterns: &[String],
    prefix: &str,
    file_count: &mut usize,
    dir_count: &mut usize,
    args: &Args
) {

    if current_depth > args.depth {
        return;
    }

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries.collect::<Result<Vec<_>, _>>().unwrap_or_default(),
        Err(_) => {
            println!("Failed to read directory: {}", path.display()); // Add this line for debugging
            return;
        },
    };

    let mut non_excluded_entries = vec![];

    for entry in entries {
        let entry_path = entry.path();
        if !should_exclude(&entry_path, exclude_patterns, gitignore_patterns, args.all, args.include_gitignore) {
            // If dironly is true, only include directories
            if !args.dironly || entry_path.is_dir() {
                non_excluded_entries.push(entry);
            }
        }
    }

    for (index, entry) in non_excluded_entries.iter().enumerate() {
        let entry = entry.path();
        let name = entry.file_name().unwrap().to_str().unwrap();
        let is_last = index == non_excluded_entries.len() - 1;

        let icon = if entry.is_dir() {
            if args.nerd_fonts { " " } else { "[DIR] " }
        } else {
            if args.nerd_fonts { " " } else { "[FILE] " }
        };

        let colored_name = if args.color {
            if entry.is_dir() {
                format!("\x1b[34m{}\x1b[0m", name)  // Blue for directories
            } else {
                format!("\x1b[32m{}\x1b[0m", name)  // Green for files
            }
        } else {
            name.to_string()
        };

        // If dironly is true and the entry is not a directory, skip it
        if args.dironly && !entry.is_dir() {
            continue;
        }

        println!(
            "{}{}{}{}",
            prefix,
            if is_last { "└── " } else { "├── " },
            icon,
            colored_name
        );

        if entry.is_dir() {
            *dir_count += 1;
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            display_tree(
                &entry,
                current_depth + 1,
                exclude_patterns,
                gitignore_patterns,
                &new_prefix,
                file_count,
                dir_count,
                &args
            );
        } else {
            *file_count += 1;
        }
    }
}
