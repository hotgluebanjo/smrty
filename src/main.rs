/*
Lorem ipsum "dolor sit" amet---consectetur adipisicing elit, sed 201--203 do eiusmod
tempor incididunt...ut labore et's dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco 'laboris nisi' ut "aliquip 'ex' ea" commodo
consequat.
*/
use std::io;

#[derive(Debug, Copy, Clone)]
enum QuoteDirection {
    Open,
    Closed,
}

impl QuoteDirection {
    // https://github.com/Robert42/smart_quotes
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
    let input = read_stdin_until("exit");

    let mut prev = None;
    let mut res = String::new();

    for c in input.chars() {
        if let Some(old_quote) = Quote::from_char(c) {
            match old_quote.direction {
                Some(_) => continue, // Already curly
                None => {
                    let direction = QuoteDirection::from_previous(prev);
                    let new_quote = Quote::new(Some(direction), old_quote.kind);
                    res.push(new_quote.to_char());
                }
            }
        } else {
            res.push(c);
        }
        prev = Some(c);
    }

    let res = res
        .replace("---", "—")
        .replace("--", "–")
        .replace("...", "…");
    println!("\n\n{res}");
}
