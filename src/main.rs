// - https://github.com/Robert42/smart_quotes
// - https://gist.github.com/maxwell-bland/59f2b09a551f91d6e38fcf9eac1a8dfb
// - https://github.com/rust-lang/book/pull/780/files#diff-9c45c870f37858b7cd69e9998520ddbfcab0c6b08e4dc32c898af283994f6153

use std::{env, io, process};

const HELP: &str = "\
smrty
  Typographic compiler.
  https://github.com/hotgluebanjo

USAGE
  smrty [OPTIONS]

  Drops into stdin. When done inputting text, enter `exit`, `quit` or a short vim write/quit command.

OPTIONS
  -h | --help        Print help
  -e | --explicit    Parse explicit LaTeX-style quotes (``double'', `single')";

#[derive(Debug, Copy, Clone)]
enum QuoteDirection {
    Open,
    Closed,
}

impl QuoteDirection {
    fn from_previous(prev: Option<char>) -> Self {
        match prev {
            Some(c) => match c {
                ' ' | '\t' | '\n' | '(' | '[' | '{' | '⟨' => Self::Open,
                _ => Self::Closed,
            },
            None => Self::Open,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum QuoteKind {
    Single,
    Double,
}

// Should be invertible.
#[derive(Debug)]
struct Quote {
    direction: Option<QuoteDirection>,
    kind: QuoteKind,
}

impl Quote {
    fn new(direction: Option<QuoteDirection>, kind: QuoteKind) -> Self {
        Self { direction, kind }
    }

    fn from_char(c: char) -> Option<Self> {
        use crate::{QuoteDirection::*, QuoteKind::*};
        match c {
            '\'' => Some(Self::new(None, Single)),
            '"' => Some(Self::new(None, Double)),
            '‘' => Some(Self::new(Some(Open), Single)),
            '’' => Some(Self::new(Some(Closed), Single)),
            '“' => Some(Self::new(Some(Open), Double)),
            '”' => Some(Self::new(Some(Closed), Double)),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        use crate::{QuoteDirection::*, QuoteKind::*};
        match (self.direction, self.kind) {
            (None, Single) => '\'',
            (None, Double) => '"',
            (Some(Open), Single) => '‘',
            (Some(Closed), Single) => '’',
            (Some(Open), Double) => '“',
            (Some(Closed), Double) => '”',
        }
    }
}

fn is_escaped(prev: Option<char>) -> bool {
    matches!(prev, Some('\\'))
}

fn smart_quotes_implicit(input: &str) -> String {
    let mut buf = String::new();
    let mut prev = None;

    for c in input.chars() {
        if let Some(old_quote) = Quote::from_char(c) {
            if is_escaped(prev) {
                buf.pop(); // Skip instead?
                buf.push(c);
                continue;
            }
            match old_quote.direction {
                Some(_) => continue, // Already curly. Possible inversion.
                None => {
                    let direction = QuoteDirection::from_previous(prev);
                    let new_quote = Quote::new(Some(direction), old_quote.kind);
                    buf.push(new_quote.to_char());
                }
            }
        } else {
            buf.push(c);
        }
        prev = Some(c);
    }

    buf
}

// Explicit quotes. Same as LaTeX.
// Avoiding regex, but that would add escaping.
fn smart_quotes_explicit(input: &str) -> String {
    input
        .replace("\"", "”")
        .replace("``", "“")
        .replace("''", "”")
        .replace("`", "‘")
        .replace("'", "’")
}

fn read_stdin_until(quit_commands: &[&str]) -> String {
    let mut buf = String::new();
    'outer: loop {
        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line);
        for q in quit_commands {
            if &line.trim() == q {
                break 'outer;
            }
        }
        buf.push_str(&line);
    }
    buf
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut explicit = false;

    for arg in args {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("{HELP}");
                process::exit(0);
            }
            "-e" | "--explicit" => explicit = true,
            _ => {
                eprintln!("Unrecognized argument: `{arg}`. Try `-h`.");
                process::exit(1);
            }
        }
    }

    let input = read_stdin_until(&["exit", "quit", ":w", ":wq", ":q"]);
    let res = if explicit {
        smart_quotes_explicit(&input)
    } else {
        smart_quotes_implicit(&input)
    };

    let res = res
        .replace("---", "—")
        .replace("--", "–")
        .replace("...", "…");

    println!("\n\n{res}");
}

/*
Lorem ipsum "dolor sit" amet---consectetur adipisicing elit, sed 201--203 do eiusmod
tempor incididunt...ut labore et's dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco 'laboris nisi' ut "aliquip 'ex' ea" commodo
consequat.

Lorem ipsum ``dolor sit'' amet---consectetur adipisicing elit, sed 201--203 do eiusmod
tempor incididunt...ut labore et's dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco `laboris nisi' ut ``aliquip `ex' ea'' commodo
consequat.
*/
#[test]
fn test_smart_quotes() {
    let implicit =
        "Lorem ipsum \"dolor sit\" amet consectetur adipisicing elit, sed 201 203 do eiusmod
tempor incididunt ut labore et's dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco 'laboris nisi' ut \"aliquip 'ex' ea\" commodo
consequat.";

    let explicit =
        "Lorem ipsum ``dolor sit'' amet consectetur adipisicing elit, sed 201 203 do eiusmod
tempor incididunt ut labore et's dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco `laboris nisi' ut ``aliquip `ex' ea\" commodo
consequat.";

    let expected =
        "Lorem ipsum “dolor sit” amet consectetur adipisicing elit, sed 201 203 do eiusmod
tempor incididunt ut labore et’s dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco ‘laboris nisi’ ut “aliquip ‘ex’ ea” commodo
consequat.";

    assert_eq!(smart_quotes_implicit(implicit), expected);
    assert_eq!(smart_quotes_explicit(explicit), expected);
}
