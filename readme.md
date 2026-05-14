# word-freq-counter

A CLI tool written in Rust that counts word frequencies from plain text or a `.txt` file — sorted by most frequent first.

Built as a Rust learning project covering `HashMap`, iterators, sorting, and module structure.

## Usage

```bash
# count words from plain text
cargo run -- "hello world hello rust hello world"

# count words from a file
cargo run -- sample.txt
```

## Output

```
hello           → 3
world           → 2
rust            → 1
```

## How it works

```
Input (text or file)
      ↓
reader.rs   — reads from file path or plain text string
      ↓
counter.rs  — cleans, splits, and counts words into a HashMap
      ↓
display.rs  — sorts by frequency and prints to terminal
```

## Project structure

```
src/
├── main.rs       — CLI arg parsing, wires everything together
├── reader.rs     — step 1: read input
├── counter.rs    — step 2: count word frequencies
└── display.rs    — step 3: sort and display output
```

## What this covers

- `HashMap` for O(1) word counting
- Iterator chaining — `split_whitespace`, `map`, `filter`, `collect`
- Sorting a `Vec<(String, usize)>` by value descending
- Module system — `mod`, `pub fn` across files
- CLI args via `std::env::args`
- File I/O via `std::fs::read_to_string`

## Run

```bash
git clone https://github.com/Souradip121/word-freq-counting-rust.git
cd word-freq-counting-rust/word-freq-counter
cargo run -- "your text here"
```