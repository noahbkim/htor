use super::cursor::ParserCursor;
use crate::parser::error::RuntimeError;

pub enum Indentation {
    Spaces(usize),
    Tabs,
}

pub struct ParserIndentation {
    indentation: Option<Indentation>,
}

fn count_at_start(
    expected: char,
    disallowed: char,
    cursor: &ParserCursor,
) -> Result<usize, RuntimeError> {
    let mut result: usize = 0;
    for c in cursor.line.chars() {
        if c == expected {
            result += 1;
        } else if c == disallowed {
            return Err(cursor.error("encountered mixed tabs and spaces".to_string()));
        } else {
            break;
        }
    }
    Ok(result)
}

impl ParserIndentation {
    pub fn new() -> Self {
        Self {
            indentation: None,
        }
    }

    pub fn determine_level(&mut self, cursor: &ParserCursor) -> Result<usize, RuntimeError> {
        match self.indentation {
            Some(Indentation::Spaces(count)) => {
                let result: usize = count_at_start(' ', '\t', &cursor)?;
                if result % count != 0 {
                    return Err(cursor.error("uneven indentation".to_string()));
                }
                Ok(result / count)
            }
            Some(Indentation::Tabs) => count_at_start('\t', ' ', &cursor),
            None => {
                let spaces: usize = count_at_start(' ', '\t', &cursor)?;
                if spaces > 0 {
                    self.indentation = Some(Indentation::Spaces(spaces));
                    return Ok(1);
                }

                let tabs: usize = count_at_start('\t', ' ', &cursor)?;
                if tabs > 0 {
                    self.indentation = Some(Indentation::Tabs);
                    return Ok(1);
                }

                return Ok(0);
            }
        }
    }
}
