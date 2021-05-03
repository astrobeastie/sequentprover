#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Formula {
    Bottom,
    Literal(String),
    Not(Box<Formula>),
    And {
        lhs: Box<Formula>,
        rhs: Box<Formula>,
    },
    Or {
        lhs: Box<Formula>,
        rhs: Box<Formula>,
    },
    Implication {
        lhs: Box<Formula>,
        rhs: Box<Formula>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Claim {
    pub lhs: Vec<Formula>,
    pub rhs: Vec<Formula>,
}

#[derive(Debug)]
pub enum ProofRule {
    Axiom,
    LBot,
    LNeg,
    RNeg,
    LAnd,
    RAnd,
    LOr,
    ROr,
    LImpl,
    RImpl,
}

#[derive(Debug)]
pub enum ProofTree {
    Open(Claim),
    Complete {
        claim: Claim,
        proof: Vec<ProofTree>,
        proof_rule: ProofRule,
    },
}

impl Formula {
    pub fn precedence(&self) -> u8 {
        match self {
            Formula::Bottom => 0,
            Formula::Literal(_) => 0,
            Formula::Not(_) => 0,
            Formula::And { lhs: _, rhs: _ } => 1,
            Formula::Or { lhs: _, rhs: _ } => 2,
            Formula::Implication { lhs: _, rhs: _ } => 3,
        }
    }
}
