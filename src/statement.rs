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
            "something.fn();",
            "int x = (a + b);",
            "int x = ((a + b));",
            "int x = ((a) + b);",
            "(int)(something.call());",
            "something.fn(\"x\" + \"y\");",
            "__a.__b = new Class(__c);",
            "String[] x= {s1,s2};",
            "a.length > 0 ? a : null;",
            "(a.length > 0) ? a : null;",
            "entry.getValue().getEntity();",
            "x.fn(a.length > 0 ? a : null);",
            "x.collect(Collectors.toMap(Map.Entry::getKey, entry -> entry.getValue().getEntity()));",
            "something.call().something.fn(\"x\" + \"y\");",
            "something.call()\n.something.fn(\"x\" + \"y\");",
            "
            somin.fn();
            ",
            "sb.append(
                \"text\"
            );
            ",
            "m.stream().map(arg -> arg);",
            "m.stream().map((value1, value2) -> value1);",
            "String x = \" nested \\\" \\\"\";",
            "sb.append('_');",
            "c = Character.toUpperCase(c);",
            "if (true) {}",
            "if (true && false) {}",
            "if (true || false) {}",
            "if (x == true) {}",
            "if (true) {} else {}",
            "if (true) {} else if(false) {} else {}",
            "if (true) {} else if(false) {}",
            "if (true) {} else if(false) {} else if(false) {}",
            "throw new IOException(\"Unable to create destination \" + destPath);",
            "com.something.MyClass myClass = new com.something.MyClass();",
            "com.something.MyClass myClass = new com.something.MyClass(123);",
            "com.something.MyClass myClass = new com.something.MyClass(something());",
            "com.something.MyClass myClass = new com.something.MyClass(\"Hello\");", 
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
