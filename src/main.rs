use std::fs;

use class::ClassDef;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::import::Import;

pub mod access_modifier;
pub mod class;
pub mod file;
pub mod ident;
pub mod import;
pub mod method;
pub mod package;
pub mod statement;
mod tests;
pub mod type_def;
// #[grammar = "ident.pest"]
#[derive(Parser)]
#[grammar = "java.pest"]
struct IdentParser;

pub trait FromNode {
    fn parse(rule: &Pair<'_, Rule>) -> Option<Self>
    where
        Self: Sized;
}

fn main() {
    let code = fs::read_to_string("./App.java").unwrap();
    let pairs = IdentParser::parse(Rule::file, code.as_str()).unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::package => {
                    println!("Package: {}", inner_pair.as_str())
                }
                Rule::imports => {
                    for imp in inner_pair.into_inner() {
                        let import = Import::parse(&imp).unwrap();
                        println!("Import:    {}", import);
                    }
                }
                Rule::class => {
                    println!("{}", ClassDef::parse(&inner_pair).unwrap())
                }
                _r => {
                    // let tty = inner_pair.as_str();
                    // println!("=={}, r={:?}", tty, r)
                }
            }
        }
    }
}
