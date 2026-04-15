use crate::units::{Unit, Operators};
use log::info;

pub enum Token {
    Number(f64),
    Unit(crate::units::Unit),
    Operator(crate::units::Operators),
}

pub fn tokenize(input: &str) {
    let characters: Vec<&str> = input.split_whitespace().collect();
    info!("characters: {:?}", characters);

    let mut tokens: Vec<Token> = Vec::new();

    for i in &characters {
        if let Ok(n) = i.parse::<f64>() {
            tokens.push(Token::Number(n))
        } else if let Some(operator) = parse_operator(i) {
            tokens.push(Token::Operator(operator));
        } else if let Some(unit) = parse_unit(unit) {
            tokens.push(Token::Unit(unit))
        } else {
            info!("unknown token: {}", i);
        }
    }
}



