use pest::iterators::Pair;
use pest::Parser;

use crate::block::bytes::translate::{bytes_from_number, integer_from_number};
use crate::error::AnonymousEvaluationError;

pub enum BytesItem {
    Expansion(String, Vec<Vec<BytesItem>>),
    Literal(Vec<u8>),
    Left,
    Right,
}

#[derive(Parser)]
#[grammar = "block/bytes/bytes.pest"]
struct BytesParser;

fn resize_anchored_right(literal: &mut Vec<u8>, size: usize) {
    literal.reverse();
    literal.resize(size, 0);
    literal.reverse();
}

fn resize_anchored_left(literal: &mut Vec<u8>, size: usize) {
    literal.resize(size, 0);
}

fn decode_number(string: &str) -> Result<Vec<u8>, AnonymousEvaluationError> {
    if string.starts_with("[") {
        let size_end: usize = string.find("]").ok_or(AnonymousEvaluationError::new(
            "invalid padding format".to_string(),
        ))?;
        let size: usize = integer_from_number(&string[1..size_end])?;
        let mut result: Vec<u8> = bytes_from_number(&string[size_end + 1..], true)?;
        resize_anchored_right(&mut result, size);
        Ok(result)
    } else if string.ends_with("]") {
        let size_start: usize = string.find("[").ok_or(AnonymousEvaluationError::new(
            "invalid padding format".to_string(),
        ))?;
        let size: usize = integer_from_number(&string[size_start + 1..string.len() - 1])?;
        let mut result: Vec<u8> = bytes_from_number(&string[..size_start], true)?;
        resize_anchored_left(&mut result, size);
        Ok(result)
    } else {
        bytes_from_number(&string, true)
    }
}

fn parse_number(pair: Pair<Rule>) -> Result<BytesItem, AnonymousEvaluationError> {
    Ok(BytesItem::Literal(decode_number(pair.as_str())?))
}

fn decode_string(string: &str) -> Result<Vec<u8>, AnonymousEvaluationError> {
    let mut result: Vec<u8> = Vec::new();

    for (i, character) in string.chars().enumerate() {
        if character > (255 as char) {
            return Err(AnonymousEvaluationError::new(format!(
                "encountered invalid character in column {}",
                i
            )));
        } else {
            result.push(character as u8);
        }
    }
    Ok(result)
}

fn parse_string(pair: Pair<Rule>) -> Result<BytesItem, AnonymousEvaluationError> {
    Ok(BytesItem::Literal(decode_string(
        pair.into_inner().next().unwrap().as_str(),
    )?))
}

fn parse_expansion(pair: Pair<Rule>) -> Result<BytesItem, AnonymousEvaluationError> {
    let mut inner_pairs = pair.into_inner();
    let name: String = String::from(inner_pairs.next().unwrap().as_str().trim_start_matches("$"));
    let mut args: Vec<Vec<BytesItem>> = Vec::new();
    for item in inner_pairs {
        args.push(parse_bytes_pair_items(item)?);
    }
    Ok(BytesItem::Expansion(name, args))
}

fn parse_bytes_pair(pair: Pair<Rule>) -> Result<BytesItem, AnonymousEvaluationError> {
    match pair.as_rule() {
        Rule::number => parse_number(pair),
        Rule::string => parse_string(pair),
        Rule::left => Ok(BytesItem::Left),
        Rule::right => Ok(BytesItem::Right),
        Rule::expansion => parse_expansion(pair),
        _ => Err(AnonymousEvaluationError::new(format!(
            "unexpected syntax tree {:?}",
            pair.as_rule()
        ))),
    }
}

fn parse_bytes_pair_items(pair: Pair<Rule>) -> Result<Vec<BytesItem>, AnonymousEvaluationError> {
    match pair.as_rule() {
        Rule::items => {
            let mut result: Vec<BytesItem> = Vec::new();
            for item in pair.into_inner() {
                result.push(parse_bytes_pair(item)?);
            }
            Ok(result)
        }
        _ => Err(AnonymousEvaluationError::new(
            "unexpected syntax tree".to_string(),
        )),
    }
}

pub fn parse_bytes(line: &str) -> Result<Vec<BytesItem>, AnonymousEvaluationError> {
    let pair: Pair<Rule> = BytesParser::parse(Rule::line, line)
        .map_err(|e| AnonymousEvaluationError::new(format!("{}", e)))?
        .next()
        .unwrap();
    parse_bytes_pair_items(pair)
}
