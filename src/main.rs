use std::collections::HashMap;
use std::env;
use std::fs;

use std::mem;

use unicode_segmentation::UnicodeSegmentation;

mod parser;

use parser::Parser;

// TODO: handle escaping quotes " within qoutes value
// TODO: handle invalid input cases. Example: URL"https::..."

/// This naive parser expects only quoted values

// #[derive(Debug)]
// enum Token {
//     Whitespace,
//     Assignment,
//     DoubleQuoteDelimiter,
//     NewlineEscape,
//     Comment, // TODO -> should not be handled withing value literal ("hello#world")

//     LiteralChar(char),
// }

// enum LiteralKind {
//     Key,
//     Value,
// }

// impl From<char> for Token {
//     fn from(c: char) -> Self {
//         match c {
//             ' ' => Token::Whitespace,
//             '=' => Token::Assignment,
//             '\n' => Token::NewlineEscape,
//             '"' => Token::DoubleQuoteDelimiter,
//             _ => Token::LiteralChar(c),
//         }
//     }
// }

fn main() {
    let contents = fs::read_to_string(".env").expect("Should have been able to read the file");
    // let mut parser = Parser::new();

    // let res = parser.parse();

    // dbg!(contents);

    let input = r#"
        HOST="localhost"
        PORT="8080"
        DEBUG="true"
    "#;

    // let res = Parser::parse(contents.as_str());
    let res = Parser::parse(input);

    dbg!(res);
}

/*

PORT =  "5432"

INNER_EQUATION="hello=there"

MULTY="ricardo
awessome"

VERSION=0.1.0

EMPTY=

*/
