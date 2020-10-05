# flushy
Rust program for aligning/flushing text files

## Installation
Install the Rust toolchain and run:
```sh
git clone https://github.com/GeneralPoxter/flushy.git
cd flushy
cargo build --release
```
`flushy` binary is located `[path to flushy]/target/release/`.  
flushy does not support Windows.

## Usage
```
flushy [FLAGS] [OPTIONS] <INPUT>

ARGS:
    <INPUT>    Path to file to format

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --columns <# cols>    Sets width of text file in columns [default: 80]
    -m, --mode <name>         Sets align/flush mode [default: left] [possible values: left, right,
                              center, flush, flush-hyphen]
    -o, --output <path>       Path to output file; input file is overwritten if not specified``
```
