use std::env;
use std::fs;

mod ast;
mod latex;
mod lexer;
mod parser;
mod proofsearch;

use ast::*;
use latex::Latex;
use lexer::lex;
use parser::Parsable;
use proofsearch::proof_search;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("supply filename as argument");
        return;
    }
    let contents = fs::read_to_string(&args[1])
        .expect("Could not read file")
        .trim_end()
        .to_owned();
    let tokens = lex(contents);
    println!("tokens:\n{:?}", tokens);
    let claim = Claim::parse(&tokens.unwrap()).unwrap();
    println!("AST:\n{:?}", claim);
    println!("Latex:\n{}", claim.latex());
    let mut tree = ProofTree::Open(claim);
    tree = proof_search(tree);
    println!("{}", tree.latex());
}
