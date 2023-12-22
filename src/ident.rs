use std::fmt::Display;

use pest::iterators::Pair;

use crate::{FromNode, Rule};

#[derive(Clone)]
pub struct Ident {
    text: String,
}

impl FromNode for Ident {
    fn parse(rule: &Pair<'_, Rule>) -> Option<Self> {
        match rule.as_rule() {
            Rule::ident => Some(Ident {
                text: rule.as_str().to_string(),
            }),
            Rule::dot => Some(Ident {
                text: rule.as_str().to_string(),
            }),
            _ => None,
        }
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Ident {
    pub fn empty() -> Self {
        Self {
            text: String::from(""),
        }
    }

    pub fn new(str: &str) -> Self {
        Self {
            text: String::from(str),
        }
    }
}

#[cfg(test)]
mod test {
    use pest::Parser;

    use crate::{FromNode, IdentParser, Rule};

    use super::Ident;

    #[test]
    fn test_grammer() {
        let test_cases = vec!["hello", "x", "SomethingElse"];

        for case in test_cases {
            let pairs = IdentParser::parse(Rule::ident, case).unwrap_or_else(|e| panic!("{}", e));

            for pair in pairs {
                // A pair is a combination of the rule which matched and a span of input
                let ident = Ident::parse(&pair);
                assert!(ident.is_some());
            }
        }
    }
}
