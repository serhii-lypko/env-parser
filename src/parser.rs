use std::collections::HashMap;
pub type ResultMap = HashMap<String, String>;

/*
    TODO:

    - use Btree
    - implement unquotes values -> space means end of value
    - implement single quoutes values
    - handle comments

    - move Token and stuff to separate files?

    - curr_literal >> better namimg?


*/

#[derive(Debug)]
pub enum ParseError {
    InvalidKey,
    UnterminatedQuote,
    MissingValue,
    InvalidAssignment,
}

#[derive(Debug)]
enum Token {
    Whitespace,
    Assignment,
    DoubleQuoteDelimiter,
    NewlineEscape,
    Comment, // TODO -> should not be handled withing value literal ("hello#world")

    LiteralChar(char),
}

enum LiteralKind {
    Key,
    Value,
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            ' ' => Token::Whitespace,
            '=' => Token::Assignment,
            '\n' => Token::NewlineEscape,
            '"' => Token::DoubleQuoteDelimiter,
            _ => Token::LiteralChar(c),
        }
    }
}

struct ParseState {
    result_map: ResultMap,

    key_buffer: Vec<char>,
    value_buffer: Vec<char>,

    // not sure about naming
    curr_literal: LiteralKind,
}

pub struct Parser;

impl Parser {
    pub fn parse<'a>(input: &'a str) -> Result<ResultMap, ParseError> {
        let mut parse_state = ParseState {
            result_map: HashMap::new(),
            key_buffer: Vec::with_capacity(32),
            value_buffer: Vec::with_capacity(64),
            curr_literal: LiteralKind::Key,
        };

        let mut chars = input.chars();

        while let Some(char) = chars.next() {
            let token: Token = char.into();

            match token {
                Token::Whitespace => {
                    if let LiteralKind::Value = parse_state.curr_literal {
                        parse_state.value_buffer.push(' ');
                    }
                }
                Token::Assignment => {
                    if parse_state.key_buffer.is_empty() {
                        return Err(ParseError::InvalidAssignment);
                    }

                    if let LiteralKind::Key = parse_state.curr_literal {
                        parse_state.curr_literal = LiteralKind::Value;
                    }
                }
                Token::DoubleQuoteDelimiter => match parse_state.curr_literal {
                    LiteralKind::Key => {
                        return Err(ParseError::InvalidKey);
                    }

                    LiteralKind::Value => {
                        if !parse_state.value_buffer.is_empty() {
                            Parser::flush_new_pair(&mut parse_state);
                        }
                    }
                },
                Token::NewlineEscape => {
                    if let LiteralKind::Value = parse_state.curr_literal {
                        if parse_state.value_buffer.is_empty() {
                            Parser::flush_new_pair(&mut parse_state);
                        } else {
                            parse_state.value_buffer.push(' ');
                        }
                    }
                }
                Token::Comment => todo!(),
                Token::LiteralChar(c) => match parse_state.curr_literal {
                    LiteralKind::Key => parse_state.key_buffer.push(c),
                    LiteralKind::Value => parse_state.value_buffer.push(c),
                },
            }
        }

        Ok(parse_state.result_map)
    }

    fn flush_new_pair(parse_state: &mut ParseState) {
        let key = String::from_iter(parse_state.key_buffer.to_owned());
        let value = String::from_iter(parse_state.value_buffer.to_owned());

        parse_state.result_map.insert(key, value);

        parse_state.key_buffer.clear();
        parse_state.value_buffer.clear();

        parse_state.curr_literal = LiteralKind::Key;
    }
}

// Tests are incomplete
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_key_value() {
        let input = r#"
            URL="https://hello.com"
        "#;

        let result = Parser::parse(input).unwrap();
        assert_eq!(result.get("URL"), Some(&"https://hello.com".to_string()));
    }

    #[test]
    fn test_multiple_key_values() {
        let input = r#"
            HOST="localhost"
            PORT="8080"
            DEBUG="true"
        "#;
        let result = Parser::parse(input).unwrap();
        assert_eq!(result.get("HOST"), Some(&"localhost".to_string()));
        assert_eq!(result.get("PORT"), Some(&"8080".to_string()));
        assert_eq!(result.get("DEBUG"), Some(&"true".to_string()));
    }

    #[test]
    fn test_empty_value() {
        let input = r#"
            URL="https://hello.com"
            EMPTY=
        "#;

        let result = Parser::parse(input).unwrap();
        assert_eq!(result.get("EMPTY"), Some(&"".to_string()));
    }

    #[test]
    fn test_multiline() {
        let input = r#"
            URL="https://hello.com"
            NOTES="docummentation
is in progress"
        "#;

        let result = Parser::parse(input).unwrap();
        assert_eq!(
            result.get("NOTES"),
            Some(&"docummentation is in progress".to_string())
        );
    }

    #[test]
    fn test_values_with_special_characters() {
        let input = r#"
            PASSWORD="p@ssw0rd!#$%"
            PATH="/usr/local/bin"
        "#;
        let result = Parser::parse(input).unwrap();
        assert_eq!(result.get("PASSWORD"), Some(&"p@ssw0rd!#$%".to_string()));
        assert_eq!(result.get("PATH"), Some(&"/usr/local/bin".to_string()));
    }

    #[test]
    fn test_values_with_spaces() {
        let input = r#"
              GREETING="Hello World"
              DESCRIPTION="This is a long description with multiple spaces"
          "#;
        let result = Parser::parse(input).unwrap();
        assert_eq!(result.get("GREETING"), Some(&"Hello World".to_string()));
        assert_eq!(
            result.get("DESCRIPTION"),
            Some(&"This is a long description with multiple spaces".to_string())
        );
    }

    #[test]
    fn test_error_missing_value() {
        let input = "KEY";
        assert!(matches!(Parser::parse(input), Err(ParseError::InvalidKey)));
    }

    #[test]
    fn test_error_invalid_assignment() {
        let input = "=VALUE";
        assert!(matches!(
            Parser::parse(input),
            Err(ParseError::InvalidAssignment)
        ));
    }

    #[test]
    fn test_multiple_empty_lines() {
        let input = r#"
            KEY1="value1"

            KEY2="value2"

            KEY3="value3"
        "#;
        let result = Parser::parse(input).unwrap();
        assert_eq!(result.get("KEY1"), Some(&"value1".to_string()));
        assert_eq!(result.get("KEY2"), Some(&"value2".to_string()));
        assert_eq!(result.get("KEY3"), Some(&"value3".to_string()));
    }

    #[test]
    fn test_value_with_equals_sign() {
        let input = r#"DATABASE_URL="postgresql://user:pass@localhost:5432/db=name""#;
        let result = Parser::parse(input).unwrap();
        assert_eq!(
            result.get("DATABASE_URL"),
            Some(&"postgresql://user:pass@localhost:5432/db=name".to_string())
        );
    }
}
