use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub enum Token {
    Bottom,
    Literal(String),
    Not,
    And,
    Or,
    Arrow,
    BigArrow,
    Comma,
    LParen,
    RParen,
}

pub fn next_token(s: &str) -> Option<(Token, String)> {
    lazy_static! {
        static ref BOT_RE: Regex = Regex::new(r"(?s)^\s*false(.*)[[:space:]]*$").unwrap();
        static ref LITERAL_RE: Regex =
            Regex::new(r"(?s)^\s*([A-Za-z][A-Za-z0-9_]*)(.*)[[:space:]]*$").unwrap();
        static ref NOT_RE: Regex = Regex::new(r"(?s)^\s*!(.*)[[:space:]]*$").unwrap();
        static ref AND_RE: Regex = Regex::new(r"(?s)^\s*&(.*)[[:space:]]*$").unwrap();
        static ref OR_RE: Regex = Regex::new(r"(?s)^\s*\|(.*)[[:space:]]*$").unwrap();
        static ref ARROW_RE: Regex = Regex::new(r"(?s)^\s*->(.*)[[:space:]]*$").unwrap();
        static ref COMMA_RE: Regex = Regex::new(r"(?s)^\s*,(.*)[[:space:]]*$").unwrap();
        static ref L_PAREN_RE: Regex = Regex::new(r"(?s)^\s*\((.*)[[:space:]]*$").unwrap();
        static ref R_PAREN_RE: Regex = Regex::new(r"(?s)^\s*\)(.*)[[:space:]]*$").unwrap();
        static ref BIG_ARROW_RE: Regex = Regex::new(r"(?s)^\s*=>(.*)[[:space:]]*$").unwrap();
    }
    if let Some(cap) = BOT_RE.captures(s) {
        return Some((Token::Bottom, cap[1].to_owned()));
    }
    if let Some(cap) = LITERAL_RE.captures(s) {
        return Some((Token::Literal(cap[1].to_owned()), cap[2].to_owned()));
    }
    if let Some(cap) = NOT_RE.captures(s) {
        return Some((Token::Not, cap[1].to_owned()));
    }
    if let Some(cap) = AND_RE.captures(s) {
        return Some((Token::And, cap[1].to_owned()));
    }
    if let Some(cap) = OR_RE.captures(s) {
        return Some((Token::Or, cap[1].to_owned()));
    }
    if let Some(cap) = ARROW_RE.captures(s) {
        return Some((Token::Arrow, cap[1].to_owned()));
    }
    if let Some(cap) = COMMA_RE.captures(s) {
        return Some((Token::Comma, cap[1].to_owned()));
    }
    if let Some(cap) = L_PAREN_RE.captures(s) {
        return Some((Token::LParen, cap[1].to_owned()));
    }
    if let Some(cap) = R_PAREN_RE.captures(s) {
        return Some((Token::RParen, cap[1].to_owned()));
    }
    if let Some(cap) = BIG_ARROW_RE.captures(s) {
        return Some((Token::BigArrow, cap[1].to_owned()));
    }

    None
}

pub fn lex(s: String) -> Option<Vec<Token>> {
    let mut res = Vec::new();
    let mut tail = s;
    while tail.len() > 0 {
        if let Some((token, newtail)) = next_token(&tail) {
            res.push(token);
            tail = newtail;
        } else {
            eprint!("failed to tokenize at:\n    \"{:?}\"\n", &tail);
            return None;
        }
    }
    Some(res)
}
