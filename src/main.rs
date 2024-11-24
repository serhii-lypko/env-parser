use std::fs;

mod parser;
use parser::Parser;

fn main() {
    let contents = fs::read_to_string(".env").expect("Should have been able to read the file");

    let res = Parser::parse(contents.as_str());

    dbg!(res);
}
