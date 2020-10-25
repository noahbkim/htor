// use std::fs::File;
// use std::io::{stdout, BufRead, BufReader, Write};
// use std::process::exit;

mod parser;
mod block;
mod evaluator;
pub mod error;

use parser::parse;
use error::EvaluationError;
use std::io::{BufReader, stdout, Write};
use std::fs::File;
use clap::{App, Arg};
use std::process::exit;


fn read(path: &str) -> Result<Vec<u8>, EvaluationError> {
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => return Err(EvaluationError::new(0, "error reading file!".to_string())),
    };

    let reader = BufReader::new(file);
    let blocks = parse(reader)?;
    Ok(Vec::new())
}

fn encode_digit(digit: u8) -> char {
    match digit {
        0..=9 => (digit + ('0' as u8)) as char,
        10..=15 => (digit - 10 + ('A' as u8)) as char,
        _ => '?',
    }
}

const DEBUG_COLUMN_WIDTH: usize = 8;
const DEBUG_COLUMN_COUNT: usize = 2;

fn debug_bytes(bytes: &Vec<u8>) -> String {
    let mut result: String = String::new();
    let mut index: usize = 0;

    for byte in bytes.iter() {
        result.push(encode_digit(byte >> 4));
        result.push(encode_digit(byte & 0xF));
        index += 1;

        if index % (DEBUG_COLUMN_WIDTH * DEBUG_COLUMN_COUNT) == 0 {
            result.push('\n');
        } else if index % DEBUG_COLUMN_WIDTH == 0 {
            result.push_str("  ");
        } else {
            result.push(' ');
        }
    }

    result
}

fn main() {
    let matches = App::new("Hex to Raw")
        .version("1.0")
        .author("Noah Kim")
        .about("Macro-assisted payload generation")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("A hex to raw script file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Prints the resultant bytes to STDIO has readable hex"),
        )
        .get_matches();

    let path = matches.value_of("file").unwrap();
    let bytes = match read(path) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{}", error);
            exit(1);
        }
    };

    if matches.is_present("debug") {
        println!("{}", debug_bytes(&bytes));
    } else {
        if let Err(e) = stdout().write_all(bytes.as_ref()) {
            eprintln!("error while writing bytes to stdout: {}", e);
            exit(1);
        }
        if let Err(e) = stdout().flush() {
            eprintln!("error while writing bytes to stdout: {}", e);
            exit(1);
        }
    }
}
