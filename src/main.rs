use std::fs::File;
use std::io::{self, IsTerminal, Read};

const USAGE: &str = "
Usage: wc [OPTION]... [FILE]...

Print newline, word, and byte counts for each FILE, and a total line if
more than one FILE is specified.  A word is a non-zero-length sequence of
printable characters delimited by white space.

With no FILE, or when FILE is -, read standard input.

The options below may be used to select which counts are printed, always in
the following order: newline, word, character, byte.
  -c, --bytes            print the byte counts
  -m, --chars            print the character counts
  -l, --lines            print the newline counts
  -w, --words            print the word counts
      --help             display this help and exit
";

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    bytes: bool,
    chars: bool,
    lines: bool,
    words: bool,
}

impl Args {
    fn parse(args: Vec<String>) -> Self {
        let (files, options): (Vec<_>, Vec<_>) = args
            .into_iter()
            .partition(|arg| arg.len() > 1 && !arg.starts_with('-') && !arg.starts_with("--"));

        let mut bytes = false;
        let mut chars = false;
        let mut lines = false;
        let mut words = false;

        // Use default options (-c -l -w) if no options are provided
        if options.is_empty() {
            bytes = true;
            lines = true;
            words = true;
        } else {
            options.iter().for_each(|option| {
                if option.starts_with("--") {
                    match option.as_str() {
                        "--bytes" => bytes = true,
                        "--chars" => chars = true,
                        "--lines" => lines = true,
                        "--words" => words = true,
                        "--help" => {
                            println!("{}", USAGE);
                            std::process::exit(0);
                        }
                        _ => {
                            eprintln!("wc: unrecognized option '{}'", option);
                            std::process::exit(1);
                        }
                    }
                } else {
                    option
                        .strip_prefix('-')
                        .unwrap()
                        .chars()
                        .for_each(|opt| match opt {
                            'c' => bytes = true,
                            'm' => chars = true,
                            'l' => lines = true,
                            'w' => words = true,
                            x => {
                                eprintln!("wc: invalid option -- '{}'", x);
                                eprintln!("Try 'wc --help' for more information.");
                                std::process::exit(1);
                            }
                        });
                }
            });
        }

        Args {
            files,
            bytes,
            chars,
            lines,
            words,
        }
    }
}

#[derive(Debug)]
struct WordCount {
    filename: String,
    bytes: usize,
    chars: usize,
    lines: usize,
    words: usize,
}

impl WordCount {
    fn parse(filename: String, input: &str, args: &Args) -> Self {
        let bytes = if args.bytes { input.len() } else { 0 };
        let chars = if args.chars { input.chars().count() } else { 0 };
        let lines = if args.lines { input.lines().count() } else { 0 };
        let words = if args.words {
            input.split_whitespace().count()
        } else {
            0
        };
        WordCount {
            filename,
            bytes,
            chars,
            lines,
            words,
        }
    }
    // TODO: calculate offset
    fn print(&self, offset: usize, args: &Args) {
        if args.lines {
            print!("{:>offset$} ", self.lines, offset = offset);
        }
        if args.words {
            print!("{:>offset$} ", self.words, offset = offset);
            //print!("{:>6} ", self.words);
        }
        if args.chars {
            print!("{:>offset$} ", self.chars, offset = offset);
            //print!("{:>6} ", self.chars);
        }
        if args.bytes {
            print!("{:>offset$} ", self.bytes, offset = offset);
            //print!("{:>6} ", self.bytes);
        }
        println!("{}", self.filename);
    }
}

fn total(results: &[Result<WordCount, String>]) -> WordCount {
    let mut bytes = 0;
    let mut chars = 0;
    let mut lines = 0;
    let mut words = 0;

    results.iter().flatten().for_each(|count| {
        bytes += count.bytes;
        chars += count.chars;
        lines += count.lines;
        words += count.words;
    });

    let filename = String::from("total");
    WordCount {
        filename,
        bytes,
        chars,
        lines,
        words,
    }
}

fn print_output(mut results: Vec<Result<WordCount, String>>, args: &Args) {
    // Find largest value to use as offset to correctly format output
    let total = total(&results);
    let max = total
        .bytes
        .max(total.chars)
        .max(total.lines)
        .max(total.words);
    let offset = max.to_string().len();

    // Append the total count if there was more than one file as input
    if results.len() > 1 {
        results.push(Ok(total));
    }

    // Print results
    results.iter().for_each(|res| match res {
        Ok(wc) => wc.print(offset, args),
        Err(e) => println!("{}", e),
    });
}

fn count(args: &Args) -> Vec<Result<WordCount, String>> {
    let mut results: Vec<Result<WordCount, String>> = Vec::new();

    // With no FILE, or when FILE is -, read standard input.
    // TODO: support interactive input which prints totals after detecting `ctrl-d`
    if args.files.is_empty() && !io::stdin().is_terminal() {
        // It would probably be more performant to use a `BufReader` instead of a String buffer,
        // but `BufRead` strips line endings that we need to include in the count.
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();

        let result = WordCount::parse("".to_string(), &buffer, args);
        results.push(Ok(result));
    } else {
        for file in &args.files {
            let mut f = match File::open(file) {
                Ok(f) => f,
                Err(_) => {
                    results.push(Err(format!("wc: {}: No such file or directory", &file)));
                    continue;
                }
            };
            let mut buffer = String::new();
            f.read_to_string(&mut buffer).expect("Unable to read file");

            let result = WordCount::parse(file.clone(), &buffer, args);
            results.push(Ok(result));
        }
    }
    results
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args = Args::parse(args);
    let results = count(&args);
    print_output(results, &args);
}
