use crate::ast::*;

pub trait Latex {
    fn latex(&self) -> String;
}

impl Latex for Formula {
    fn latex(&self) -> String {
        match self {
            Formula::Bottom => r"\bot".to_owned(),
            Formula::Literal(s) => s.to_owned(),
            Formula::Not(f) => {
                let mut s = String::new();
                s.push_str(r"\neg ");
                if self.precedence() < (*f).precedence() {
                    s.push_str(r"\left(");
                }
                s.push_str(&(*f).latex());
                if self.precedence() < (*f).precedence() {
                    s.push_str(r"\right)");
                }
                s
            }
            Formula::And { lhs, rhs } => {
                let mut s = String::new();
                if self.precedence() < (*lhs).precedence() {
                    s.push_str(r"\left(");
                }
                s.push_str(&(*lhs).latex());
                if self.precedence() < (*lhs).precedence() {
                    s.push_str(r"\right)");
                }
                s.push_str(r" \wedge ");
                if self.precedence() < (*rhs).precedence() {
                    s.push_str(r"\left(");
                }
                s.push_str(&(*rhs).latex());
                if self.precedence() < (*rhs).precedence() {
                    s.push_str(r"\right)");
                }
                s
            }
            Formula::Or { lhs, rhs } => {
                let mut s = String::new();
                if self.precedence() < (*lhs).precedence() {
                    s.push_str(r"\left(");
                }
                s.push_str(&(*lhs).latex());
                if self.precedence() < (*lhs).precedence() {
                    s.push_str(r"\right)");
                }
                s.push_str(r" \vee ");
                if self.precedence() < (*rhs).precedence() {
                    s.push_str(r"\left(");
                }
                s.push_str(&(*rhs).latex());
                if self.precedence() < (*rhs).precedence() {
                    s.push_str(r"\right)");
                }
                s
            }
            Formula::Implication { lhs, rhs } => {
                let mut s = String::new();
                if self.precedence() <= (*lhs).precedence() {
                    s.push_str(r"\left(");
                }
                s.push_str(&(*lhs).latex());
                if self.precedence() <= (*lhs).precedence() {
                    s.push_str(r"\right)");
                }
                s.push_str(r" \rightarrow ");
                if self.precedence() < (*rhs).precedence() {
                    s.push_str(r"\left(");
                }
                s.push_str(&(*rhs).latex());
                if self.precedence() < (*rhs).precedence() {
                    s.push_str(r"\right)");
                }
                s
            }
        }
    }
}

impl Latex for Claim {
    fn latex(&self) -> String {
        let mut s = String::new();
        if self.lhs.len() > 0 {
            s.push_str(&self.lhs[0].latex());
            for f in &self.lhs[1..] {
                s.push_str(", ");
                s.push_str(&f.latex());
            }
        }
        s.push_str(r" \Rightarrow ");
        if self.rhs.len() > 0 {
            s.push_str(&self.rhs[0].latex());
            for f in &self.rhs[1..] {
                s.push_str(", ");
                s.push_str(&f.latex());
            }
        }

        s
    }
}

impl Latex for ProofRule {
    fn latex(&self) -> String {
        match self {
            ProofRule::Axiom => r"Ax".to_owned(),
            ProofRule::LBot => r"\bot L".to_owned(),
            ProofRule::LNeg => r"\neg L".to_owned(),
            ProofRule::RNeg => r"\neg R".to_owned(),
            ProofRule::LAnd => r"\wedge L".to_owned(),
            ProofRule::RAnd => r"\wedge R".to_owned(),
            ProofRule::LOr => r"\vee L".to_owned(),
            ProofRule::ROr => r"\vee R".to_owned(),
            ProofRule::LImpl => r"\rightarrow L".to_owned(),
            ProofRule::RImpl => r"\rightarrow R".to_owned(),
        }
    }
}

impl Latex for ProofTree {
    fn latex(&self) -> String {
        match self {
            ProofTree::Open(claim) => claim.latex(),
            ProofTree::Complete {
                claim,
                proof,
                proof_rule,
            } => {
                let mut s = String::new();
                s.push_str(r"\frac{");
                let proof_strings: Vec<String> = proof.iter().map(|t| t.latex()).collect();
                s.push_str(&proof_strings.join(r"\quad "));
                s.push_str(r"}{");
                s.push_str(&claim.latex());
                s.push_str(r"}\quad ");
                s.push_str(&proof_rule.latex());
                s
            }
        }
    }
}
