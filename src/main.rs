use anyhow::{Result, bail};
use clap::{Parser, ValueEnum};
use std::io::stdin;
use std::iter;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Case {
    LowerCamel,
    UpperCamel,
    Snake,
    ScreamingSnake,
    Kebab,
    ScreamingKebab,
}

/// Read newline delimited names from stdin, transform to the given case and write them to stdout
///
/// If you are running the program in "interactive" mode (you did not pipe input into the program),
/// use Ctrl+D to close stdin and quit.
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// The case to be read
    pub case: Case,
}

fn main() -> Result<()> {
    let args = Args::parse();

    for line in stdin().lines() {
        match tokenize(line?.as_str()) {
            Ok(tokens) => println!("{}", recombine(&tokens, args.case)),
            Err(e) => eprintln!("{e}"),
        }
    }

    Ok(())
}

fn tokenize(input: &str) -> Result<Vec<String>> {
    if input.chars().any(|ch| !ch.is_ascii()) {
        bail!("skipping non-ascii input '{input}'")
    }

    let split_indices: Vec<usize> = input
        .chars()
        .zip(input.char_indices().skip(1))
        .filter_map(|(left, (idx, right))| {
            if should_split(left, right) {
                Some(idx)
            } else {
                None
            }
        })
        .collect();

    let mut input = input;
    let mut tokens = Vec::with_capacity(split_indices.len() + 1);
    for index in split_indices.into_iter().rev() {
        let (left, right) = input.split_at(index);
        input = left;
        tokens.push(normalize(right));
    }
    tokens.push(normalize(input));
    tokens.reverse();
    Ok(tokens)
}

fn recombine(tokens: &[String], format: Case) -> String {
    match format {
        Case::LowerCamel => {
            if let Some((first, rest)) = tokens.split_first() {
                iter::once(first.to_owned())
                    .chain(rest.into_iter().map(|t| capitalize_first(t)))
                    .collect()
            } else {
                String::new()
            }
        }
        Case::UpperCamel => tokens.into_iter().map(|t| capitalize_first(t)).collect(),
        Case::Snake => tokens.join("_"),
        Case::ScreamingSnake => tokens.join("_").to_ascii_uppercase(),
        Case::Kebab => tokens.join("-"),
        Case::ScreamingKebab => tokens.join("-").to_uppercase(),
    }
}

fn should_split(left: char, right: char) -> bool {
    return left.is_ascii_lowercase() && right.is_ascii_uppercase()
        || !is_terminator(left) && is_terminator(right);
}

fn is_terminator(c: char) -> bool {
    matches!(c, '_' | '-' | ' ')
}

fn normalize(token: &str) -> String {
    token
        .trim()
        .chars()
        .filter(|&c| !is_terminator(c))
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

// Return a copy of this token with the first character capitalized
fn capitalize_first(token: &str) -> String {
    let mut chars_iter = token.chars();
    let first = chars_iter.next();

    match first {
        Some(ch) => iter::once(ch.to_ascii_uppercase())
            .chain(chars_iter)
            .collect(),
        None => String::new(),
    }
}
