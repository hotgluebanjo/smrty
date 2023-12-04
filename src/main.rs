// - https://github.com/Robert42/smart_quotes
// - https://gist.github.com/maxwell-bland/59f2b09a551f91d6e38fcf9eac1a8dfb
// - https://github.com/rust-lang/book/pull/780/files#diff-9c45c870f37858b7cd69e9998520ddbfcab0c6b08e4dc32c898af283994f6153

use std::{io, env, process};

const HELP: &'static str = "\
smrty
  Typographic compiler.
  https://github.com/hotgluebanjo

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

// Theoretically this type should allow two-way conversion.
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
        match c {
            '\'' => Some(Self::new(None, QuoteKind::Single)),
            '"' => Some(Self::new(None, QuoteKind::Double)),
            '‘' => Some(Self::new(Some(QuoteDirection::Open), QuoteKind::Single)),
            '’' => Some(Self::new(Some(QuoteDirection::Closed), QuoteKind::Single)),
            '“' => Some(Self::new(Some(QuoteDirection::Open), QuoteKind::Double)),
            '”' => Some(Self::new(Some(QuoteDirection::Closed), QuoteKind::Double)),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match (self.direction, self.kind) {
            (None, QuoteKind::Single) => '\'',
            (None, QuoteKind::Double) => '"',
            (Some(QuoteDirection::Open), QuoteKind::Single) => '‘',
            (Some(QuoteDirection::Closed), QuoteKind::Single) => '’',
            (Some(QuoteDirection::Open), QuoteKind::Double) => '“',
            (Some(QuoteDirection::Closed), QuoteKind::Double) => '”',
        }
    }
}

fn smart_quotes_implicit(input: &str) -> String {
    let mut buf = String::new();
    let mut prev = None;

    for c in input.chars() {
        if let Some(old_quote) = Quote::from_char(c) {
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

fn smart_quotes_explicit(input: &str) -> String {
    input
        .replace("\"", "”")
        .replace("``", "“")
        .replace("''", "”")
        .replace("`", "‘")
        .replace("'", "’")
}

fn read_stdin_until(quit_command: &'static str) -> String {
    let mut buf = String::new();
    loop {
        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line);
        if line.trim() == quit_command {
            break;
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
                eprintln!("Unrecognized argument: `{arg}`. Check `-h`.");
                process::exit(1);
            }
        }
    }

    let input = read_stdin_until("exit");
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
    let implicit = "Lorem ipsum \"dolor sit\" amet consectetur adipisicing elit, sed 201 203 do eiusmod
tempor incididunt ut labore et's dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco 'laboris nisi' ut \"aliquip 'ex' ea\" commodo
consequat.";

    let explicit = "Lorem ipsum ``dolor sit'' amet consectetur adipisicing elit, sed 201 203 do eiusmod
tempor incididunt ut labore et's dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco `laboris nisi' ut ``aliquip `ex' ea\" commodo
consequat.";

    let expected = "Lorem ipsum “dolor sit” amet consectetur adipisicing elit, sed 201 203 do eiusmod
tempor incididunt ut labore et’s dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco ‘laboris nisi’ ut “aliquip ‘ex’ ea” commodo
consequat.";

    assert_eq!(smart_quotes_implicit(implicit), expected);
    assert_eq!(smart_quotes_explicit(explicit), expected);
}
