# pkfinder

A fast, multithreaded command-line tool, written in Rust, for profiling delimited data files (CSV, TSV, pipe-delimited and so on) before you load them into a database.

Given a file, pkfinder can:

- **Find a primary key**: the smallest combination of columns that uniquely identifies each row, or that leaves as few duplicates as possible.
- **Check a specific column set**: count the duplicates for a combination you choose, and optionally write them to a file for inspection.
- **Infer column types**: suggest SQL-style types (`int`, `numeric(p, s)`, `char`, `varchar(n)`) and nullability for each column. It also flags values in scientific notation, a common sign that a file has been round-tripped through Excel.
- **Detect the delimiter automatically**, or use one you specify.

It uses [rayon](https://github.com/rayon-rs/rayon) to search column combinations in parallel. pkfinder can also be built as a Python extension module via [PyO3](https://pyo3.rs).

## AI disclosure
No AI was used in the making of this program. It was all hand-written. AI (Claude) was used to generate this README for when I made this repo available for public viewing.

## Building

Requires a Rust toolchain ([rustup](https://rustup.rs)).

```bash
cargo build --release
```

The binary is written to `target/release/pkfinder` (`pkfinder.exe` on Windows).

## Usage

```
pkfinder [OPTIONS] <FILE> [SUBCOMMAND]
```

With no subcommand, pkfinder looks for a primary key combination.

```bash
pkfinder data.csv
```

### Global options

| Option | Description |
| --- | --- |
| `-p, --properties` | Print file details (delimiter, column and row counts) |
| `-c, --column-numbers` | Print each column number alongside its header |
| `-s, --skip <N>` | Skip the first N lines (for non-CSV preamble at the top of a file) |
| `-t, --threads <N>` | Number of CPU threads to use (default: 2) |
| `--delimiter <CHAR>` | Set the delimiter instead of detecting it (`\t` for tab) |
| `-d, --debug` | Print the total run time |

### `pk`: find a primary key

```bash
# Find the smallest primary key combination and show the column numbers
pkfinder data.csv pk -n

# Find the columns that can't be dropped without adding duplicates first
# (faster when the key has many columns)
pkfinder data.csv pk -c

# Write duplicate counts for every combination of up to 3 columns to a file
pkfinder data.csv pk -o combos.csv -m 3
```

> **Note:** pkfinder picks the smallest combination that minimizes duplicates. That's a statistical answer, not a semantic one, so check the result against what the data actually means before using it as a real key.

### `cs`: check a column set

```bash
# Count duplicates when columns 1, 3 and 4 are used as the key
pkfinder data.csv cs 1 3 4

# Also write the duplicate rows to a file
pkfinder data.csv cs 1 3 4 -o duplicates.csv
```

Column numbers start at 1. Run `pkfinder data.csv -c` to list them.

### `typer`: infer column types

```bash
pkfinder data.csv typer          # suggested type for each column
pkfinder data.csv typer -d       # detailed stats per column
pkfinder data.csv typer -t       # trim whitespace before analyzing
```

## Python

The crate also exposes a `PKFinder` class to Python. Build it with [maturin](https://www.maturin.rs):

```bash
pip install maturin
maturin develop --release
```

```python
from pkfinder import PKFinder

finder = PKFinder("data.csv")   # optional: delimiter, skip_lines
print(finder.get_pk())          # list of header names
print(finder.get_known_keys())
```
