use std::fs::{File};
use std::io::{BufRead, BufReader, Lines, Write};
use std::env;
use std::process;
use std::error::Error;
use std::fmt;
use std::collections::HashMap;
use std::iter::Enumerate;
use std::process::{Command, Stdio};


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
        write!(f, "line {}: {}", self.line + 1, self.what)
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
    lines: Enumerate<Lines<BufReader<File>>>,
}

impl ParserCursor {
    pub fn new(lines: Enumerate<Lines<BufReader<File>>>) -> Self {
        ParserCursor { line: String::new(), line_number: 0, lines }
    }

    pub fn advance(&mut self) -> Result<bool, ParserError> {
        match self.lines.next() {
            None => Ok(false),
            Some((line_number, line_result)) => match line_result {
                Err(_) => Err(ParserError::new("failed to read line", line_number)),
                Ok(line) => {
                    self.line = line;
                    self.line_number = line_number;
                    Ok(true)
                },
            },
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
            let result: usize = count_at_start(' ', '\t', &cursor)?;
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

fn decode_letter(cursor: &ParserCursor, letter: &char) -> Result<u8, ParserError> {
    match letter {
        '0'..='9' => Ok((*letter as u8) - ('0' as u8)),
        'A'..='F' => Ok((*letter as u8) - ('A' as u8) + 10),
        'a'..='f' => Ok((*letter as u8) - ('a' as u8) + 10),
        _ => Err(ParserError::new("invalid hex digit", cursor.line_number))
    }
}

fn decode_bytes(cursor: &ParserCursor, string: &str) -> Result<Vec<u8>, ParserError> {
    let mut result: Vec<u8> = Vec::new();
    if string.len() % 2 != 0 {
        Err(ParserError::new("hex word has odd length", cursor.line_number))
    } else {
        let collected: Vec<char> = string.chars().collect();
        for i in 0..(collected.len() / 2) {
            let high: u8 = decode_letter(cursor, collected.get(i * 2).unwrap())?;
            let low: u8 = decode_letter(cursor, collected.get(i * 2 + 1).unwrap())?;
            result.push((high << 4) + low);
        }
        Ok(result)
    }
}

fn get_defined(cursor: &ParserCursor, context: &mut ParserContext, name: &str) -> Result<Vec<u8>, ParserError> {
    return match context.definitions.get(name) {
        None => Err(ParserError::new(format!("no definition for {}", name).as_str(), cursor.line_number)),
        Some(result) => Ok(result.clone()),
    }
}

fn parse_bytes(cursor: &mut ParserCursor, context: &mut ParserContext) -> Result<Vec<u8>, ParserError> {
    let mut result: Vec<u8> = Vec::new();
    for word in cursor.line.split_ascii_whitespace() {
        match word.chars().next() {
            None => {},
            Some(char) => result.extend(match char {
                '$' => get_defined(cursor, context, word.trim_start_matches('$'))?,
                _ => decode_bytes(cursor, word)?,
            }),
        };
    }
    Ok(result)
}

fn parse_repeat(args: Vec<&str>, cursor: &mut ParserCursor, context: &mut ParserContext, level: usize) -> Result<Vec<u8>, ParserError> {
    match args.first() {
        None => Err(ParserError::new("expected exactly one argument indicating repetition count", cursor.line_number)),
        Some(arg) => {
            if !cursor.advance()? {
                return Ok(Vec::<u8>::new())
            }
            let count: usize = arg.parse::<usize>().map_err(|_e| ParserError::new(format!("invalid repetition count {}", arg).as_str(), cursor.line_number))?;
            let contents: Vec<u8> = parse(cursor, context, level + 1)?;
            Ok(contents.repeat(count))
        }
    }
}

fn parse_define(args: Vec<&str>, cursor: &mut ParserCursor, context: &mut ParserContext, level: usize) -> Result<Vec<u8>, ParserError> {
    match args.first() {
        None => return Err(ParserError::new("expected exactly one argument indicating definition name", cursor.line_number)),
        Some(arg) => {
            if !cursor.advance()? {
                return Ok(Vec::<u8>::new())
            }
            let name: String = arg.to_string();
            let contents: Vec<u8> = parse(cursor, context, level + 1)?;
            context.definitions.insert(name, contents);
        }
    }
    Ok(Vec::<u8>::new())
}

fn on_indentation_level(cursor: &ParserCursor, context: &mut ParserContext, level: usize) -> Result<bool, ParserError> {
    let indentation_level: usize = infer_indentation_level(cursor, context)?;
    if indentation_level < level {
        Ok(false)
    } else if indentation_level > level {
        Err(ParserError::new(format!("expected indentation {}, found {}", level, indentation_level).as_str(), cursor.line_number))
    } else {
        Ok(true)
    }
}

fn parse_raw(cursor: &mut ParserCursor, context: &mut ParserContext, level: usize) -> Result<String, ParserError> {
    let mut result: String = String::new();
    while on_indentation_level(cursor, context, level)? {
        result.push_str(cursor.line.trim_start());
        result.push('\n');
        if !cursor.advance()? {
            break;
        }
    }
    Ok(result)
}

fn parse_assembly(_args: Vec<&str>, cursor: &mut ParserCursor, context: &mut ParserContext, level: usize) -> Result<Vec<u8>, ParserError> {
    if !cursor.advance()? {
        Ok(Vec::<u8>::new())
    } else {
        let assembly = parse_raw(cursor, context, level + 1)?;
        let file = tempfile::NamedTempFile::new().map_err(|e| ParserError::new(format!("error creating temporary file: {}", e).as_str(), cursor.line_number))?;
        let mut child = Command::new("gcc")
            .arg("-c")  // Compile assembly
            .arg("-o")  // Output file path
            .arg(file.path().to_str().unwrap())
            .arg("-x")  // Read from STDIN
            .arg("assembler")
            .arg("-")
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| ParserError::new(format!("failed to run gcc: {}", e).as_str(), cursor.line_number))?;

        {
            let stdin = child.stdin.as_mut().ok_or(ParserError::new("failed to communicate with gcc process", cursor.line_number))?;
            stdin.write_all(assembly.as_bytes()).map_err(|e| ParserError::new(format!("failed to write assembly to gcc pipe: {}", e).as_str(), cursor.line_number))?;
        }

        let output = child.wait_with_output().map_err(|e| ParserError::new(format!("error while awaiting gcc: {}", e).as_str(), cursor.line_number))?;
        if !output.status.success() {
            return Err(ParserError::new("compilation of assembly failed", cursor.line_number));
        }
        let file = elf::File::open_path(file.path())
            .map_err(|e| ParserError::new(format!("failed to open elf file: {:?}", e).as_str(), cursor.line_number))?;
        let text = match file.get_section(".text") {
            Some(section) => section,
            None => return Err(ParserError::new("failed to find .text in elf file", cursor.line_number)),
        };
        Ok(text.data.clone())
    }
}

fn parse(mut cursor: &mut ParserCursor, context: &mut ParserContext, level: usize) -> Result<Vec<u8>, ParserError>  {
    let mut result: Vec<u8> = Vec::new();
    while on_indentation_level(cursor, context, level)? {
        cursor.line = String::from(cursor.line.trim());
        if cursor.line.starts_with("@") {
            let line: String = cursor.line.clone();
            let words: Vec<&str> = line.split_ascii_whitespace().collect();
            let (head, tail) = words.split_at(1);
            match head.first() {
                None => return Err(ParserError::new("macro lines must contain a command before the colon", cursor.line_number)),
                Some(&"@repeat") => result.extend(parse_repeat(tail.to_vec(), cursor, context, level)?),
                Some(&"@define") => result.extend(parse_define(tail.to_vec(), cursor, context, level)?),
                Some(&"@assembly") => result.extend(parse_assembly(tail.to_vec(), cursor, context, level)?),
                Some(command) => return Err(ParserError::new(format!("unknown command: {}", command).as_str(), cursor.line_number)),
            };
        } else {
            result.extend(parse_bytes(cursor, context)?)
        }

        if !cursor.advance()? {
            break;
        }
    }
    Ok(result)
}

fn read(path: &str) -> Result<Vec<u8>, ParserError> {
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => return Err(ParserError::new("error reading file!", 0)),
    };

    let reader: BufReader<File> = BufReader::new(file);
    let mut cursor: ParserCursor = ParserCursor::new(reader.lines().enumerate());
    let mut context: ParserContext = ParserContext::new();
    cursor.advance()?;
    parse(&mut cursor, &mut context, 0)
}


fn encode_digit(digit: u8) -> char {
    match digit {
        0..=9 => (digit + ('0' as u8)) as char,
        10..=15 => (digit - 10 + ('A' as u8)) as char,
        _ => '?',
    }
}

fn debug_bytes(bytes: &Vec<u8>) -> String {
    let mut result: String = String::new();
    for byte in bytes.iter() {
        result.push_str("0x");
        result.push(encode_digit(byte >> 4));
        result.push(encode_digit(byte & 0xF));
        result.push(' ');
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("must supply a file path");
        process::exit(1);
    }

    match read(&args[1]) {
        Ok(result) => {
            eprintln!("{}", debug_bytes(&result));
            let mut string: String = String::new();
            result.iter().for_each(|byte| string.push(*byte as char));
            println!("{}", string);
            process::exit(0);
        },
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    }
}
