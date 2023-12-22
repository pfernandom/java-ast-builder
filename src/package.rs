use std::fmt::Display;

use pest::iterators::Pair;

use crate::{FromNode, Rule};

pub struct PackageDef {
    pub text: String,
}

impl FromNode for PackageDef {
    fn parse(rule: &Pair<'_, Rule>) -> Option<Self> {
        match rule.as_rule() {
            Rule::package => {
                let mut pkg = PackageDef::empty();
                for inner in rule.clone().into_inner() {
                    match inner.as_rule() {
                        Rule::object => pkg.text = inner.as_str().to_string(),
                        _ => {}
                    }
                }

                Some(pkg)
            }
            _ => None,
        }
    }
}

impl Display for PackageDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "package {};", self.text)
    }
}

impl PackageDef {
    pub fn empty() -> Self {
        Self {
            text: String::from(""),
        }
    }
}

#[cfg(test)]
mod test {
    use pest::Parser;

    use crate::{FromNode, IdentParser, Rule};

    use super::PackageDef;

    #[test]
    fn test_grammer() {
        let test_cases = vec![
            "package com.x.something;",
            "package com.x_1.something;",
            "package something;",
        ];

        for case in test_cases {
            let pairs = IdentParser::parse(Rule::package, case).unwrap_or_else(|e| panic!("{}", e));

            for pair in pairs {
                // A pair is a combination of the rule which matched and a span of input
                let package = PackageDef::parse(&pair);
                assert!(package.is_some());
                println!("{}", package.unwrap());
            }
        }
    }
}
