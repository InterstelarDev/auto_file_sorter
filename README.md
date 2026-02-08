# afs - (Auto File Sorter)

A simple Rust CLI tool that sorts folders by extension.

## Features:
- File sort by extension
- Runs in specified directory

## Installation:
```bash

git clone https://github.com/InterstelarDev/auto_file_sorter.git

cargo build --release

./target/release/afs --path ~/Downloads 
```

**Current Version:** 0.1.3
* Fixed issue to ensure folders are created within the specified directory.