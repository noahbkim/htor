use std::fs::{File};
use std::io::{BufRead, BufReader, Lines};
use std::env;
use std::process;
use std::error::Error;
use std::fmt;
use std::collections::HashMap;
use std::iter::Enumerate;


#[derive(Debug)]
pub struct ParserError {
    what: String,
    line: usize,
}

impl ParserError {
    pub fn new(what: &str, line: usize) -> Self {
        Self {
            what: String::from(what),
            line
        }
    }
}

impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.what)
    }
}

enum Indentation {
    Spaces(usize),
    Tabs,
}

struct ParserContext {
    indentation: Option<Indentation>,
    definitions: HashMap<String, Vec<u8>>,
}

impl ParserContext {
    pub fn new() -> Self {
        Self { indentation: None, definitions: HashMap::new() }
    }
}

struct ParserCursor {
    line: String,
    line_number: usize,
}

impl ParserCursor {
    pub fn new() -> Self {
        ParserCursor { line: String::new(), line_number: 0 }
    }

    pub fn consume(&mut self, lines: &mut Enumerate<Lines<BufReader<File>>>) -> Option<Result<(), ParserError>> {
        match lines.next() {
            None => None,
            Some((line_number, line_result)) => Some(match line_result {
                Err(_) => Err(ParserError::new("failed to read line", line_number)),
                Ok(line) => {
                    self.line = line;
                    self.line_number = line_number;
                    Ok(())
                },
            }),
        }
    }
}

fn count_at_start(expected: char, disallowed: char, cursor: &ParserCursor) -> Result<usize, ParserError> {
    let mut result: usize = 0;
    for c in cursor.line.chars() {
        if c == expected {
            result += 1;
        } else if c == disallowed {
            return Err(ParserError::new("encountered mixed tabs and spaces", cursor.line_number))
        } else {
            break;
        }
    }
    Ok(result)
}

fn infer_indentation_level(cursor: &ParserCursor, context: &mut ParserContext) -> Result<usize, ParserError> {
    match context.indentation {
        Some(Indentation::Spaces(count)) => {
            let mut result: usize = count_at_start(' ', '\t', &cursor)?;
            if result % count != 0 {
                return Err(ParserError::new("incorrect indentation", cursor.line_number))
            }
            Ok(result / count)
        },
        Some(Indentation::Tabs) => {
            count_at_start('\t', ' ', &cursor)
        },
        None => {
            let spaces: usize = count_at_start(' ', '\t', &cursor)?;
            if spaces > 0 {
                context.indentation = Some(Indentation::Spaces(spaces));
                return Ok(1)
            }

            let tabs: usize = count_at_start('\t', ' ', &cursor)?;
            if tabs > 0 {
                context.indentation = Some(Indentation::Tabs);
                return Ok(1)
            }

            return Ok(0)
        }
    }
}

fn parse(mut cursor: &mut ParserCursor, lines: &mut Enumerate<Lines<BufReader<File>>>, context: &mut ParserContext, level: usize) -> Result<Vec<u8>, ParserError>  {
    let result: Vec<u8> = Vec::new();
    loop {
        let indentation_level: usize = infer_indentation_level(cursor, context)?;
        if indentation_level < level {
            break;
        } else if indentation_level > level {
            return Err(ParserError::new("unexpected indentation", cursor.line_number));
        }

        cursor.line = String::from(cursor.line.trim());
        if cursor.line.starts_with("repeat") {

        }

        cursor.consume(lines);
    }
    Ok(result)
}

fn read(path: &str) -> Result<Vec<u8>, ParserError> {
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => return Err(ParserError::new("error reading file!", 0)),
    };

    let reader: BufReader<File> = BufReader::new(file);
    let mut cursor: ParserCursor = ParserCursor::new();
    let mut lines: Enumerate<Lines<BufReader<File>>> = reader.lines().enumerate();
    cursor.consume(&mut lines);

    let mut context: ParserContext = ParserContext::new();
    parse(&mut cursor, &mut lines, &mut context, 0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("must supply a file path");
        process::exit(1);
    }

    match read(&args[1]) {
        Ok(result) => {
            println!("{:x?}", result);
            process::exit(0);
        },
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    }
}
