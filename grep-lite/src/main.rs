use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn get_file(file_name: &str) -> File {
    File::open(file_name).unwrap()
}

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("file")
                .help("File to search")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("context")
                .help("Number of lines of surrounding context to display with matches, max of 5. If not specified, defaults to 0.")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("file").unwrap();
    let file = get_file(input);
    let reader = BufReader::new(file);

    let context_arg = args.value_of("context");

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

    if tags.is_empty() {
        return;
    }

    let file = get_file(input);
    let reader = BufReader::new(file);

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
