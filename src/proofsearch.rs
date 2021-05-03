use crate::ast::*;

pub fn apply_proof_rule(claim: &Claim, rule: ProofRule) -> ProofTree {
    match rule {
        ProofRule::Axiom => {
            for f in &claim.lhs {
                if claim.rhs.iter().any(|g| g.eq(&f)) {
                    return ProofTree::Complete {
                        claim: claim.clone(),
                        proof: vec![],
                        proof_rule: rule,
                    };
                }
            }
            ProofTree::Open(claim.clone())
        }
        ProofRule::LBot => {
            if claim.lhs.iter().any(|g| g.eq(&Formula::Bottom)) {
                ProofTree::Complete {
                    claim: claim.clone(),
                    proof: vec![],
                    proof_rule: rule,
                }
            } else {
                ProofTree::Open(claim.clone())
            }
        }
        ProofRule::LNeg => {
            for (i, f) in claim.lhs.iter().enumerate() {
                match f {
                    Formula::Not(inner) => {
                        let mut new_claim = Claim {
                            lhs: claim.lhs.clone(),
                            rhs: claim.rhs.clone(),
                        };
                        new_claim.lhs.remove(i);
                        new_claim.rhs.push(*inner.clone());
                        return ProofTree::Complete {
                            claim: claim.clone(),
                            proof: vec![ProofTree::Open(new_claim)],
                            proof_rule: rule,
                        };
                    }
                    _ => continue,
                }
            }
            ProofTree::Open(claim.clone())
        }
        ProofRule::RNeg => {
            for (i, f) in claim.rhs.iter().enumerate() {
                match f {
                    Formula::Not(inner) => {
                        let mut new_claim = Claim {
                            lhs: claim.lhs.clone(),
                            rhs: claim.rhs.clone(),
                        };
                        new_claim.rhs.remove(i);
                        new_claim.lhs.push(*inner.clone());
                        return ProofTree::Complete {
                            claim: claim.clone(),
                            proof: vec![ProofTree::Open(new_claim)],
                            proof_rule: rule,
                        };
                    }
                    _ => continue,
                }
            }
            ProofTree::Open(claim.clone())
        }
        ProofRule::LAnd => {
            for (i, f) in claim.lhs.iter().enumerate() {
                match f {
                    Formula::And { lhs, rhs } => {
                        let mut new_claim = Claim {
                            lhs: claim.lhs.clone(),
                            rhs: claim.rhs.clone(),
                        };
                        new_claim.lhs.remove(i);
                        new_claim.lhs.push(*lhs.clone());
                        new_claim.lhs.push(*rhs.clone());
                        return ProofTree::Complete {
                            claim: claim.clone(),
                            proof: vec![ProofTree::Open(new_claim)],
                            proof_rule: rule,
                        };
                    }
                    _ => continue,
                }
            }
            ProofTree::Open(claim.clone())
        }
        ProofRule::RAnd => {
            for (i, f) in claim.rhs.iter().enumerate() {
                match f {
                    Formula::And { lhs, rhs } => {
                        let mut new_claim1 = Claim {
                            lhs: claim.lhs.clone(),
                            rhs: claim.rhs.clone(),
                        };
                        let mut new_claim2 = Claim {
                            lhs: claim.lhs.clone(),
                            rhs: claim.rhs.clone(),
                        };
                        new_claim1.rhs.remove(i);
                        new_claim2.rhs.remove(i);
                        new_claim1.rhs.push(*lhs.clone());
                        new_claim2.rhs.push(*rhs.clone());
                        return ProofTree::Complete {
                            claim: claim.clone(),
                            proof: vec![ProofTree::Open(new_claim1), ProofTree::Open(new_claim2)],
                            proof_rule: rule,
                        };
                    }
                    _ => continue,
                }
            }
            ProofTree::Open(claim.clone())
        }
        ProofRule::LOr => {
            for (i, f) in claim.lhs.iter().enumerate() {
                match f {
                    Formula::Or { lhs, rhs } => {
                        let mut new_claim1 = Claim {
                            lhs: claim.lhs.clone(),
                            rhs: claim.rhs.clone(),
                        };
                        let mut new_claim2 = Claim {
                            lhs: claim.lhs.clone(),
                            rhs: claim.rhs.clone(),
                        };
                        new_claim1.lhs.remove(i);
                        new_claim2.lhs.remove(i);
                        new_claim1.lhs.push(*lhs.clone());
                        new_claim2.lhs.push(*rhs.clone());
                        return ProofTree::Complete {
                            claim: claim.clone(),
                            proof: vec![ProofTree::Open(new_claim1), ProofTree::Open(new_claim2)],
                            proof_rule: rule,
                        };
                    }
                    _ => continue,
                }
            }
            ProofTree::Open(claim.clone())
        }
        ProofRule::ROr => {
            for (i, f) in claim.rhs.iter().enumerate() {
                match f {
                    Formula::Or { lhs, rhs } => {
                        let mut new_claim = Claim {
                            lhs: claim.lhs.clone(),
                            rhs: claim.rhs.clone(),
                        };
                        new_claim.rhs.remove(i);
                        new_claim.rhs.push(*lhs.clone());
                        new_claim.rhs.push(*rhs.clone());
                        return ProofTree::Complete {
                            claim: claim.clone(),
                            proof: vec![ProofTree::Open(new_claim)],
                            proof_rule: rule,
                        };
                    }
                    _ => continue,
                }
            }
            ProofTree::Open(claim.clone())
        }
        ProofRule::LImpl => {
            for (i, f) in claim.lhs.iter().enumerate() {
                match f {
                    Formula::Implication { lhs, rhs } => {
                        let mut new_claim1 = Claim {
                            lhs: claim.lhs.clone(),
                            rhs: claim.rhs.clone(),
                        };
                        let mut new_claim2 = Claim {
                            lhs: claim.lhs.clone(),
                            rhs: claim.rhs.clone(),
                        };
                        new_claim1.lhs.remove(i);
                        new_claim2.lhs.remove(i);
                        new_claim1.rhs.push(*lhs.clone());
                        new_claim2.lhs.push(*rhs.clone());
                        return ProofTree::Complete {
                            claim: claim.clone(),
                            proof: vec![ProofTree::Open(new_claim1), ProofTree::Open(new_claim2)],
                            proof_rule: rule,
                        };
                    }
                    _ => continue,
                }
            }
            ProofTree::Open(claim.clone())
        }
        ProofRule::RImpl => {
            for (i, f) in claim.rhs.iter().enumerate() {
                match f {
                    Formula::Implication { lhs, rhs } => {
                        let mut new_claim = Claim {
                            lhs: claim.lhs.clone(),
                            rhs: claim.rhs.clone(),
                        };
                        new_claim.rhs.remove(i);
                        new_claim.lhs.push(*lhs.clone());
                        new_claim.rhs.push(*rhs.clone());
                        return ProofTree::Complete {
                            claim: claim.clone(),
                            proof: vec![ProofTree::Open(new_claim)],
                            proof_rule: rule,
                        };
                    }
                    _ => continue,
                }
            }
            ProofTree::Open(claim.clone())
        }
    }
}

pub fn proof_search(tree: ProofTree) -> ProofTree {
    if let ProofTree::Open(claim) = &tree {
        let search_order = vec![
            ProofRule::LBot,
            ProofRule::Axiom,
            ProofRule::LNeg,
            ProofRule::RNeg,
            ProofRule::LAnd,
            ProofRule::ROr,
            ProofRule::RImpl,
            ProofRule::RAnd,
            ProofRule::LOr,
            ProofRule::LImpl,
        ];
        for rule in search_order {
            let new_tree = apply_proof_rule(claim, rule);
            if let ProofTree::Complete {
                claim,
                mut proof,
                proof_rule,
            } = new_tree
            {
                let new_proofs = proof.drain(..).map(|t| proof_search(t)).collect();
                return ProofTree::Complete {
                    claim: claim,
                    proof: new_proofs,
                    proof_rule: proof_rule,
                };
            }
        }
    }
    tree
}
