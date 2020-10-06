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
    <INPUT>    Path to input file

FLAGS:
    -d, --double     Breaks on double new lines if specified
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --columns <# cols>    Sets width of text in columns [default: 80]
    -m, --mode <name>         Sets align/flush mode [default: left] [possible values: left, right,
                              center, flush, flush-hyphen]
    -o, --output <path>       Path to output file; overwrites input file if not specified
```

**Warning: this process is not reversible!**  
It is recommended that an output file be specified if input file is not backed up.
