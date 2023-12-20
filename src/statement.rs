use std::fmt::Display;

use pest::iterators::Pair;

use crate::{FromNode, Rule};

pub struct Statement {
    text: String,
}

impl FromNode for Statement {
    fn parse(rule: &Pair<'_, Rule>) -> Option<Self> {
        match rule.as_rule() {
            Rule::stmt => Some(Statement {
                text: rule.as_str().to_string(),
            }),
            _ => None,
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Statement {
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

    use super::Statement;

    #[test]
    fn test_grammer() {
        let test_cases = vec![
            "String x = \"Hello\";",
            "return \"Hello\";",
            "String myString = new String(\"hello\");",
            "something.fn.call(\"Hello\");",
        ];

        for case in test_cases {
            let pairs = IdentParser::parse(Rule::stmt, case).unwrap_or_else(|e| panic!("{}", e));

            for pair in pairs {
                // A pair is a combination of the rule which matched and a span of input
                let stmt = Statement::parse(&pair);
                assert!(stmt.is_some());

                println!("{}", stmt.unwrap());
            }
        }
    }
}
