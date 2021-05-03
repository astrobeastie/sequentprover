use crate::ast;
use crate::lexer::Token;

#[derive(Debug)]
enum Primary {
    Bottom,
    Literal(String),
    Negation(Box<Primary>),
    Group(Box<Formula>),
}

#[derive(Debug)]
enum L2Formula {
    Single(Box<Primary>),
    And {
        lhs: Box<Primary>,
        rhs: Box<L2Formula>,
    },
}

#[derive(Debug)]
enum L3Formula {
    Single(Box<L2Formula>),
    Or {
        lhs: Box<L2Formula>,
        rhs: Box<L3Formula>,
    },
}

#[derive(Debug)]
enum L4Formula {
    Single(Box<L3Formula>),
    Implication {
        lhs: Box<L3Formula>,
        rhs: Box<L4Formula>,
    },
}

#[derive(Debug)]
struct Formula(Box<L4Formula>);

pub trait Parsable {
    fn parse_partial(tokens: &[Token]) -> Option<(Self, &[Token])>
    where
        Self: Sized;
    fn parse(tokens: &[Token]) -> Option<Self>
    where
        Self: Sized,
    {
        match Self::parse_partial(tokens) {
            None => None,
            Some((f, t)) => {
                if t.len() == 0 {
                    Some(f)
                } else {
                    None
                }
            }
        }
    }
}

impl Parsable for Formula {
    fn parse_partial(tokens: &[Token]) -> Option<(Formula, &[Token])> {
        match L4Formula::parse_partial(tokens) {
            Some((f, t)) => Some((Formula(Box::new(f)), t)),
            _ => None,
        }
    }
}

impl Parsable for Primary {
    fn parse_partial(tokens: &[Token]) -> Option<(Primary, &[Token])> {
        if tokens.len() == 0 {
            None
        } else {
            match &tokens[0] {
                Token::Bottom => Some((Primary::Bottom, &tokens[1..])),
                Token::Literal(s) => Some((Primary::Literal(s.to_string()), &tokens[1..])),
                Token::Not => {
                    let operand = Primary::parse_partial(&tokens[1..]);
                    match operand {
                        None => None,
                        Some((f, t)) => Some((Primary::Negation(Box::new(f)), t)),
                    }
                }
                Token::LParen => {
                    let inner = Formula::parse_partial(&tokens[1..]);
                    match inner {
                        None => None,
                        Some((f, t)) => {
                            if t.len() == 0 {
                                None
                            } else {
                                match t[0] {
                                    Token::RParen => Some((Primary::Group(Box::new(f)), &t[1..])),
                                    _ => None,
                                }
                            }
                        }
                    }
                }
                _ => None,
            }
        }
    }
}

impl Parsable for L2Formula {
    fn parse_partial(tokens: &[Token]) -> Option<(L2Formula, &[Token])> {
        match Primary::parse_partial(tokens) {
            None => None,
            Some((f, t)) => {
                if t.len() > 0 {
                    match &t[0] {
                        Token::And => {
                            let rhs = L2Formula::parse_partial(&t[1..]);
                            match rhs {
                                None => None,
                                Some((f2, t2)) => Some((
                                    L2Formula::And {
                                        lhs: Box::new(f),
                                        rhs: Box::new(f2),
                                    },
                                    t2,
                                )),
                            }
                        }
                        _ => Some((L2Formula::Single(Box::new(f)), t)),
                    }
                } else {
                    Some((L2Formula::Single(Box::new(f)), t))
                }
            }
        }
    }
}

impl Parsable for L3Formula {
    fn parse_partial(tokens: &[Token]) -> Option<(L3Formula, &[Token])> {
        match L2Formula::parse_partial(tokens) {
            None => None,
            Some((f, t)) => {
                if t.len() > 0 {
                    match &t[0] {
                        Token::Or => {
                            let rhs = L3Formula::parse_partial(&t[1..]);
                            match rhs {
                                None => None,
                                Some((f2, t2)) => Some((
                                    L3Formula::Or {
                                        lhs: Box::new(f),
                                        rhs: Box::new(f2),
                                    },
                                    t2,
                                )),
                            }
                        }
                        _ => Some((L3Formula::Single(Box::new(f)), t)),
                    }
                } else {
                    Some((L3Formula::Single(Box::new(f)), t))
                }
            }
        }
    }
}

impl Parsable for L4Formula {
    fn parse_partial(tokens: &[Token]) -> Option<(L4Formula, &[Token])> {
        match L3Formula::parse_partial(tokens) {
            None => None,
            Some((f, t)) => {
                if t.len() > 0 {
                    match &t[0] {
                        Token::Arrow => {
                            let rhs = L4Formula::parse_partial(&t[1..]);
                            match rhs {
                                None => None,
                                Some((f2, t2)) => Some((
                                    L4Formula::Implication {
                                        lhs: Box::new(f),
                                        rhs: Box::new(f2),
                                    },
                                    t2,
                                )),
                            }
                        }
                        _ => Some((L4Formula::Single(Box::new(f)), t)),
                    }
                } else {
                    Some((L4Formula::Single(Box::new(f)), t))
                }
            }
        }
    }
}

impl Parsable for Vec<ast::Formula> {
    fn parse_partial(tokens: &[Token]) -> Option<(Vec<ast::Formula>, &[Token])> {
        let mut res: Vec<ast::Formula> = Vec::new();
        let mut tail = &tokens[..];
        while let Some((f, t)) = Formula::parse_partial(tail) {
            res.push(f.into());
            if t.len() > 0 {
                match t[0] {
                    Token::Comma => {
                        tail = &t[1..];
                    }
                    _ => {
                        tail = &t[..];
                        break;
                    }
                }
            } else {
                tail = &t[..];
                break;
            }
        }
        Some((res, tail))
    }
}

impl Parsable for ast::Claim {
    fn parse_partial(tokens: &[Token]) -> Option<(ast::Claim, &[Token])> {
        let (lhs, t) = Vec::<ast::Formula>::parse_partial(tokens).unwrap();
        if t.len() == 0 {
            None
        } else {
            match t[0] {
                Token::BigArrow => {
                    let (rhs, t2) = Vec::<ast::Formula>::parse_partial(&t[1..]).unwrap();
                    Some((ast::Claim { lhs: lhs, rhs: rhs }, t2))
                }
                _ => None,
            }
        }
    }
}

impl<T> From<Box<T>> for ast::Formula
where
    T: Into<ast::Formula>,
{
    fn from(f: Box<T>) -> Self {
        (*f).into()
    }
}

impl From<Formula> for ast::Formula {
    fn from(f: Formula) -> Self {
        match f {
            Formula(inner) => (*inner).into(),
        }
    }
}

impl From<Primary> for ast::Formula {
    fn from(f: Primary) -> Self {
        match f {
            Primary::Bottom => ast::Formula::Bottom,
            Primary::Literal(s) => ast::Formula::Literal(s),
            Primary::Negation(f) => ast::Formula::Not(Box::new(f.into())),
            Primary::Group(f) => f.into(),
        }
    }
}

impl From<L2Formula> for ast::Formula {
    fn from(f: L2Formula) -> Self {
        match f {
            L2Formula::Single(f) => f.into(),
            L2Formula::And { lhs, rhs } => ast::Formula::And {
                lhs: Box::new(lhs.into()),
                rhs: Box::new(rhs.into()),
            },
        }
    }
}

impl From<L3Formula> for ast::Formula {
    fn from(f: L3Formula) -> Self {
        match f {
            L3Formula::Single(f) => f.into(),
            L3Formula::Or { lhs, rhs } => ast::Formula::Or {
                lhs: Box::new(lhs.into()),
                rhs: Box::new(rhs.into()),
            },
        }
    }
}

impl From<L4Formula> for ast::Formula {
    fn from(f: L4Formula) -> Self {
        match f {
            L4Formula::Single(f) => f.into(),
            L4Formula::Implication { lhs, rhs } => ast::Formula::Implication {
                lhs: Box::new(lhs.into()),
                rhs: Box::new(rhs.into()),
            },
        }
    }
}
