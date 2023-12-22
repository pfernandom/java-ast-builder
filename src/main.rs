use std::{collections::HashMap, fs};

use crate::import::Import;
use class::ClassDef;
use file::FileDef;
use glob::glob;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

pub mod access_modifier;
pub mod attribute;
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
    // let mut file_map = HashMap::new();
    let mut class_map = HashMap::new();
    let mut import_map = HashMap::new();

    for entry in glob("./examples/**/*.java").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let path_str = path.as_path().to_str().unwrap();

                println!("{:?}", path_str);

                let code = fs::read_to_string(path_str).unwrap();

                let pairs = IdentParser::parse(Rule::file, code.as_str())
                    .unwrap_or_else(|e| panic!("{}", e));

                for pair in pairs {
                    // A pair is a combination of the rule which matched and a span of input
                    let maybe_file = FileDef::parse(&pair);
                    assert!(maybe_file.is_some());

                    let file = maybe_file.unwrap();
                    println!("{}", file);

                    // let imports = file.imports;
                    import_map.insert(path_str.to_string(), file.imports());

                    // let ns_classes = file.get_namespaced_classes();

                    for (class_path, class_def) in file.get_namespaced_classes() {
                        class_map.insert(class_path, class_def);
                    }
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }

    println!("Entries: {}", class_map.len());

    // for class in file_map.values() {
    //     class.

    // }
}
