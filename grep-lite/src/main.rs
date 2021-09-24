use clap::{App, Arg};
use regex::Regex;
use std::io::{prelude::*, BufReader};
use std::{fs::File, io};

fn get_file(file_name: &str) -> File {
    File::open(file_name).unwrap()
}

fn process_lines_without_context<T: BufRead + Sized>(reader: T, re: Regex) {
    let mut line_num = 0;

    for line_ in reader.lines() {
        line_num += 1;
        let line = line_.unwrap();
        if re.find(&line).is_some() {
            println!("{}:\t {}", line_num, line)
        }
    }
}

fn tag_matches<T: BufRead + Sized>(
    reader: T,
    re: Regex,
    ctx_lines: usize,
) -> (Vec<usize>, Vec<Vec<String>>) {
    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<String>> = vec![];

    for (i, line_) in reader.lines().enumerate() {
        let line = line_.unwrap();
        let contains_substring = re.find(&line);

        if contains_substring.is_some() {
            tags.push(i);
            ctx.push(Vec::with_capacity(2 * ctx_lines + 1));
        }
    }

    (tags, ctx)
}

fn process_file_lines<T: BufRead + Sized>(
    reader: T,
    ctx_lines: usize,
    tags: Vec<usize>,
    mut ctx: Vec<Vec<String>>,
) {
    for (i, line_) in reader.lines().enumerate() {
        let line = line_.unwrap();

        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                if i == *tag {
                    ctx[j].push(format!("{}:\t{}", i, String::from(&line)));
                } else {
                    ctx[j].push(format!("{}-\t{}", i, String::from(&line)));
                }
            }
        }
    }

    for (i, local_ctx) in ctx.iter().enumerate() {
        if i > 0 {
            println!("--");
        }

        for x in local_ctx.iter() {
            println!("{}", x);
        }
    }
}

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("PATTERN")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true)
            .index(1))
        .arg(Arg::with_name("FILE")
            .short("f")
            .long("file")
            .help("File to search")
            .takes_value(true)
            .required(false))
        .arg(Arg::with_name("CONTEXT")
            .short("c")
            .long("context")
            .help("Number of lines of surrounding context to display with matches, max of 5. If not specified, defaults to 0.")
            .takes_value(true)
            .required(false))
        .get_matches();

    let pattern = args.value_of("PATTERN").unwrap();
    let re = Regex::new(pattern).unwrap();

    let context_arg = args.value_of("CONTEXT");

    let ctx_lines: usize = match context_arg {
        Some(c) => match c.parse::<usize>() {
            Ok(us) => {
                if us > 5 {
                    println!("Invalid 'context' switch value '{}'. Maximum is 5", us);
                    std::process::exit(1);
                }
                us
            }
            Err(e) => {
                println!("Invalid 'context' switch value '{}': {}", c, e);
                std::process::exit(1);
            }
        },
        // default to 0
        None => 0,
    };

    let input = args.value_of("FILE").unwrap_or("-");

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines_without_context(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        let (tags, ctx) = tag_matches(reader, re, ctx_lines);

        if tags.is_empty() {
            return;
        }

        let file = get_file(input);
        let reader = BufReader::new(file);

        process_file_lines(reader, ctx_lines, tags, ctx)
    };
}
