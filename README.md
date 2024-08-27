# artt

## What is artt?

Artt stands for *"Another Rust Tree Tool"*. It's a command-line utility to show a directory tree. Yes, I know, there's a million tools to do this already, but my favorite tool is [tree](https://manpages.ubuntu.com/manpages/trusty/man1/tree.1.html) included with many linux distributions. But I primarily work on Windows and there's not many great tools that mimic the behavior of the linux tree command.

So this tool is being built as a (mostly) drop-in replacement for tree but on Windows. It's being built with [Rust](https://www.rust-lang.org/), using [Clap](https://docs.rs/clap/latest/clap/index.html) for command-line argument parsing. Artt will include many of the same features as tree, but some features will not be supported, and there will be a couple features in artt that are not in tree. See the compatibility table/roadmap below.

## Examples

(WIP)

## Installation

1. Clone the repository

    ```bash
    git clone https://github.com/tmorgan497/artt.git
    ```

2. Install Rust

3. Build artt

    ```bash
    cd artt
    cargo build --release
    ```

4. Run artt

    ```bash
    cd artt
    ./target/release/artt ./ --help
    ```

5. (Optional) Set an alias or add artt to your path (platform-specific)

6. (Optional) Install a [Nerd Font](https://www.nerdfonts.com/) so artt can display icons for directories and files

- Note: In the future, pre-built binaries will be available for most platforms.

## Usage

(WIP)

- Run `artt --help` to see the full list of options.

## Roadmap/Features

| Feature | Tree | Artt | Tree Option | Artt Option |
| --- | --- | --- | --- | --- |
| Color Output | ✔ | ✔ | -C | -C |
| Hidden Files/All | ✔ | ✔ | -a | |
| Directories Only | ✔ | Future | -d |  |
| No Report | ✔ | Future | --noreport | |
| Icons (Nerd Fonts) | ✖ | ✔ |  | -b |
| Depth/Level | ✔ | ✔ | -L | -L |
| Ignore | ✔ | ✔+ (supports list of patterns) | -I | -I |
| Symlinks | ✔ | Future | -l | - |
| Sort | ✔ | Future | (multiple) | |
| Full Path | ✔ | Future | -f |  |
| Pattern Matching | ✔ | Future | -p | |
| File output | ✔ | Future | -o |  |
| Stats | ✔ | Future | multiple |  |
| Auto-ignore .gitignore files | ✖ | Future | | |
