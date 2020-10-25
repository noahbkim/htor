use crate::error::AnonymousEvaluationError;

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
    line: &String,
) -> Result<usize, AnonymousEvaluationError> {
    let mut result: usize = 0;
    for c in line.chars() {
        if c == expected {
            result += 1;
        } else if c == disallowed {
            return Err(AnonymousEvaluationError::new("encountered mixed tabs and spaces".to_string()));
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

    pub fn determine(&mut self, line: &String) -> Result<usize, AnonymousEvaluationError> {
        match self.indentation {
            Some(Indentation::Spaces(count)) => {
                let result: usize = count_at_start(' ', '\t', line)?;
                if result % count != 0 {
                    Err(AnonymousEvaluationError::new("uneven indentation".to_string()))
                } else {
                    Ok(result / count)
                }
            }
            Some(Indentation::Tabs) => count_at_start('\t', ' ', line),
            None => {
                let spaces: usize = count_at_start(' ', '\t', line)?;
                if spaces > 0 {
                    self.indentation = Some(Indentation::Spaces(spaces));
                    return Ok(1);
                }

                let tabs: usize = count_at_start('\t', ' ', line)?;
                if tabs > 0 {
                    self.indentation = Some(Indentation::Tabs);
                    return Ok(1);
                }

                return Ok(0);
            }
        }
    }

    pub fn eq(&mut self, line: &String, level: usize) -> Result<bool, AnonymousEvaluationError> {
        let indentation_level: usize = self.determine(line)?;
        eprintln!("found {} expected {}", indentation_level, level);
        if indentation_level > level {
            Err(AnonymousEvaluationError::new("unexpected indentation".to_string()))
        } else if indentation_level == level {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn ge(&mut self, line: &String, level: usize) -> Result<bool, AnonymousEvaluationError> {
        let indentation_level: usize = self.determine(line)?;
        Ok(indentation_level >= level)
    }

    pub fn trim(&self, line: &String, level: usize) -> String {
        line.chars().skip(match self.indentation {
            Some(Indentation::Spaces(count)) => count * level,
            Some(Indentation::Tabs) => level,
            None => 0
        }).collect::<String>()
    }
}
