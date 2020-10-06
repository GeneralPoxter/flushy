use clap::Clap;
use std::convert::Infallible;
use std::format;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Align {
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Flush {
    Flush,
    FlushHyphen,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    AlignMode(Align),
    FlushMode(Flush),
}

impl FromStr for Mode {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Mode, Self::Err> {
        Ok(match input {
            "left" => Mode::AlignMode(Align::Left),
            "center" => Mode::AlignMode(Align::Center),
            "right" => Mode::AlignMode(Align::Right),
            "flush" => Mode::FlushMode(Flush::Flush),
            "flush-hyphen" => Mode::FlushMode(Flush::FlushHyphen),
            _ => Mode::AlignMode(Align::Left),
        })
    }
}

#[derive(Clap, Clone, Debug, PartialEq, Eq)]
#[clap(
    version = "1.0.0",
    author = "poxter <generalpoxter@gmail.com>",
    about = "Aligns/flushes text files"
)]
struct Opts {
    #[clap(
        short = 'c',
        long = "columns",
        about = "Sets width of text in columns",
        takes_value = true,
        value_name = "# cols",
        default_value = "80"
    )]
    col: usize,
    #[clap(
        short = 'm',
        long = "mode",
        about = "Sets align/flush mode",
        takes_value = true,
        value_name = "name",
        possible_values = &["left", "right", "center", "flush", "flush-hyphen"],
        default_value = "left"
    )]
    mode: String,
    #[clap(
        short = 'o',
        long = "output",
        about = "Path to output file; overwrites input file if not specified",
        takes_value = true,
        value_name = "path"
    )]
    output: Option<String>,
    #[clap(
        short = 'd',
        long = "double",
        about = "Breaks on double new lines if specified"
    )]
    double: bool,
    #[clap(
        about = "Path to input file",
        value_name = "INPUT",
        required = true
    )]
    input: String,
}

fn length(x: &str) -> usize {
    return x.chars().count();
}

fn format_line(line: &str, col: usize, last_space: usize, mode: Mode) -> String {
    if last_space == 0 {
        return line.to_owned();
    }

    match mode {
        Mode::AlignMode(align) => {
            let mut tmp = String::new();
            let tmp_txt = &line[..last_space];

            match align {
                Align::Left => {
                    tmp.push_str(tmp_txt);
                }

                Align::Center => {
                    tmp.push_str(&" ".repeat((col - last_space) / 2));
                    tmp.push_str(tmp_txt);
                    tmp.push_str(&" ".repeat(col - length(&tmp)));
                }

                Align::Right => {
                    tmp.push_str(&" ".repeat(col - last_space));
                    tmp.push_str(tmp_txt);
                }
            }

            return tmp;
        }

        Mode::FlushMode(flush) => {
            let mut tmp = line.to_owned();

            match flush {
                Flush::Flush => {
                    return tmp;
                }

                Flush::FlushHyphen => {
                    if length(line) == col && line.chars().last().unwrap().is_alphabetic() {
                        tmp = tmp[..col - 1].to_owned();
                        if line.chars().nth(col - 2).unwrap().is_alphabetic() {
                            tmp.push_str("-");
                        }
                    }
                    return tmp;
                }
            }
        }
    }
}

fn format(line: &str, col: usize, mode: Mode) -> String {
    let mut out = Vec::<String>::new();
    let mut cur = String::new();
    let mut last_space = 0;
    let mut curlen = 0;

    for c in line.chars() {
        if curlen == 0 && c != ' ' || curlen > 0 {
            if c == ' ' {
                last_space = cur.bytes().len(); // use length in bytes for slicing
            }
            cur.push(c);
            curlen += 1;
        }

        if curlen == col {
            out.push(format_line(&cur, col, last_space, mode));

            match mode {
                Mode::AlignMode(_align) => {
                    if last_space == 0 || last_space == col - 1 {
                        cur = String::new();
                        curlen = 0;
                    } else {
                        cur = cur[(last_space + 1)..].to_owned();
                        curlen = curlen - last_space - 1;
                    }
                }

                Mode::FlushMode(flush) => match flush {
                    Flush::Flush => {
                        cur = String::new();
                        curlen = 0;
                    }

                    Flush::FlushHyphen => {
                        let last_char = cur.chars().last().unwrap();
                        if last_char.is_alphabetic() {
                            cur = last_char.to_string();
                            curlen = 1;
                        } else {
                            cur = String::new();
                            curlen = 0;
                        }
                    }
                },
            }

            last_space = 0;
        }
    }

    if length(&cur) > 0 {
        last_space = curlen;
        out.push(format_line(&cur, col, last_space, mode));
    }

    return out.join("\n");
}

fn main() {
    let opts: Opts = Opts::parse();
    let mode = Mode::from_str(&opts.mode).unwrap();

    let mut tmp = File::create("/tmp/flushy.txt").expect("Couldn't create /tmp/flushy.txt");
    let reader =
        BufReader::new(File::open(&opts.input).expect(&format!("Couldn't read {}", opts.input)));

    let mut line_cum = Vec::<String>::new();

    for line in reader.lines() {
        let line_parse = line.unwrap();

        if opts.double {
            line_cum.push(line_parse.clone());
            if length(&line_parse) == 0 {
                writeln!(tmp, "{}\n", format(&line_cum.join(" "), opts.col, mode))
                    .expect("Couldn't write to /tmp/flushy.txt");
                line_cum = Vec::<String>::new();
            }
        } else {
            let line_format = format(&line_parse, opts.col, mode);
            writeln!(tmp, "{}", line_format).expect("Couldn't write to /tmp/flushy.txt");
        }
    }

    if line_cum.len() > 0 {
        writeln!(tmp, "{}", format(&line_cum.join(" "), opts.col, mode))
            .expect("Couldn't write to /tmp/flushy.txt");
    }

    let out = opts.output.unwrap_or(opts.input);

    fs::copy("/tmp/flushy.txt", &out).expect(&format!("Couldn't write to {}", out));
}
