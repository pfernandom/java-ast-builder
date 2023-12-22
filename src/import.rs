use std::fmt::Display;

use pest::iterators::Pair;

use crate::{FromNode, Rule};

#[derive(Clone)]
pub struct Import {
    text: String,
}

impl FromNode for Import {
    fn parse(rule: &Pair<'_, Rule>) -> Option<Self> {
        match rule.as_rule() {
            Rule::import => Some(Import {
                text: rule.as_str().to_string(),
            }),
            _ => None,
        }
    }
}

impl Display for Import {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Import {
    pub fn empty() -> Self {
        Self {
            text: String::from(""),
        }
    }
}

#[cfg(test)]
mod test {
    use pest::Parser;

    use crate::{import::Import, FromNode, IdentParser, Rule};

    #[test]
    fn test_grammer() {
        let test_cases = vec!["import com.x.Something;", "import com.x_1.Something;"];

        for case in test_cases {
            let pairs = IdentParser::parse(Rule::import, case).unwrap_or_else(|e| panic!("{}", e));

            for pair in pairs {
                // A pair is a combination of the rule which matched and a span of input
                let ident = Import::parse(&pair);
                assert!(ident.is_some());
            }
        }
    }
}
