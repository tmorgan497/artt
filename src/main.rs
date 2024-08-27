/* src\main.rs
Command-line utility to display a directory tree with exclusions
*/

use clap::Parser;
use std::fs;
use std::path::Path;


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
}

fn main() {
    let args = Args::parse();
    let mut file_count = 0;
    let mut dir_count = 0;

    if args.debug {
        println!(".");
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

        println!("No-report: {}", args.noreport);
    }

    let exclude_patterns = parse_exclude_patterns(&args.ignore);
    display_tree(
        Path::new(&args.dir),
        args.depth,
        0,
        &exclude_patterns,
        "",
        &mut file_count,
        &mut dir_count,
        args.dironly,
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

fn should_exclude(path: &Path, exclude_patterns: &[String], include_hidden: bool) -> bool {
    let path_str = path.to_string_lossy().replace("\\", "/"); // Normalize path

    if !include_hidden && path.file_name().unwrap().to_str().unwrap().starts_with('.') {
        return true;
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
    max_depth: usize,
    current_depth: usize,
    exclude_patterns: &[String],
    prefix: &str,
    file_count: &mut usize,
    dir_count: &mut usize,
    include_hidden: bool,
    args: &Args
) {

    if current_depth > max_depth {
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
        if !should_exclude(&entry_path, exclude_patterns, include_hidden) {
            non_excluded_entries.push(entry);
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
                max_depth,
                current_depth + 1,
                exclude_patterns,
                &new_prefix,
                file_count,
                dir_count,
                include_hidden,
                &args
            );
        } else {
            *file_count += 1;
        }
    }
}
