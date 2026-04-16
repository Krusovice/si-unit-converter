use crate::units::{Unit, Operators};
use log::info;

#[derive(Debug)]
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
        } else if let Some(unit) = parse_unit(i) {
            tokens.push(Token::Unit(unit))
        } else if let Some(operator) = parse_operator(i) {
            tokens.push(Token::Operator(operator));
        } else {
            info!("unknown token: {}", i);
        }
    info!("Tokens {:?}", tokens);
    }
}

fn parse_unit(s: &str) -> Option<Unit> {
    match s {
        "N" => Some(Unit::Newton),
        "kN" => Some(Unit::KiloNewton),
        "MN" => Some(Unit::MegaNewton),
        "GN" => Some(Unit::GigaNewton),
        "mm" => Some(Unit::Millimeter),
        "cm" => Some(Unit::CentiMeter),
        "m" => Some(Unit::Meter),
        "km" => Some(Unit::KiloMeter),
        "mm2" => Some(Unit::SquareMilliMeter),
        "cm2" => Some(Unit::SquareCentiMeter),
        "m2" => Some(Unit::SquareMeter),
        "km2" => Some(Unit::SquareKiloMeter),
        "Pa" => Some(Unit::Pascal),
        "kPa" => Some(Unit::KiloPascal),
        "MPa" => Some(Unit::MegaPascal),
        "GPa" => Some(Unit::GigaPascal),
        "s" => Some(Unit::Second),
        "h" => Some(Unit::Hour),
        "g" => Some(Unit::Gram),
        "kg" => Some(Unit::KiloGram),
        _ => None,
    }
}

fn parse_operator(s: &str) -> Option<Operators> {
    match s {
        "+" => Some(Operators::Plus),
        "-" => Some(Operators::Minus),
        "*" => Some(Operators::Multiply),
        "/" => Some(Operators::Divide),
        "^" => Some(Operators::Power),
        _ => None,
    }
}
