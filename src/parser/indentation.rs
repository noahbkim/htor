use super::cursor::ParserCursor;
use super::error::ParserError;

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
) -> Result<usize, ParserError> {
    let mut result: usize = 0;
    for c in cursor.line.chars() {
        if c == expected {
            result += 1;
        } else if c == disallowed {
            return Err(ParserError::new(
                "encountered mixed tabs and spaces",
                cursor.line_number,
            ));
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

    pub fn determine_level(&mut self, cursor: &ParserCursor) -> Result<usize, ParserError> {
        match self.indentation {
            Some(Indentation::Spaces(count)) => {
                let result: usize = count_at_start(' ', '\t', &cursor)?;
                if result % count != 0 {
                    return Err(ParserError::new("uneven indentation", cursor.line_number));
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
