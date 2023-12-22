use std::fmt::Display;

use pest::iterators::Pair;

use crate::{access_modifier::AccessModifier, ident::Ident, FromNode, Rule};

#[derive(Clone)]
pub struct MethodDef {
    name: Ident,
    static_modifier: bool,
    access_modifier: AccessModifier,
}

impl FromNode for MethodDef {
    fn parse(rule: &Pair<'_, Rule>) -> Option<Self> {
        match rule.as_rule() {
            Rule::method => {
                let mut method = MethodDef::empty();

                for inner in rule.clone().into_inner() {
                    match inner.as_rule() {
                        Rule::access_modifier => {
                            method.access_modifier = AccessModifier::from(inner.as_str());
                        }
                        Rule::ident => method.name = Ident::parse(&inner).unwrap(),
                        Rule::static_modifier => method.static_modifier = true,
                        // Rule::args => method.static_modifier = true,
                        _ => {}
                    }
                }

                Some(method)
            }
            _ => None,
        }
    }
}

impl Display for MethodDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},
            access_modifier: {}",
            self.name, self.access_modifier
        )
    }
}

impl MethodDef {
    pub fn empty() -> Self {
        Self {
            name: Ident::empty(),
            access_modifier: AccessModifier::Default,
            static_modifier: false,
        }
    }
}

#[cfg(test)]
mod test {
    use pest::Parser;

    use crate::{method::MethodDef, FromNode, IdentParser, Rule};

    #[test]
    fn test_grammer() {
        let test_cases = vec![
            (
                "public static void main(String[] arg) { }",
                "public",
                true,
                "main",
            ),
            (
                "private static void main(String[] arg) { }",
                "private",
                true,
                "main",
            ),
            (
                "public void main(String[] arg) { }",
                "public",
                false,
                "main",
            ),
            (
                "private void main(String[] arg) { }",
                "private",
                false,
                "main",
            ),
            (
                "private void main(String[] arg) throws Exception { }",
                "private",
                false,
                "main",
            ),
            ("private void my_fn() { }", "private", false, "my_fn"),
        ];

        for (case, access_mod, static_mod, fn_name) in test_cases {
            let pairs = IdentParser::parse(Rule::method, case).unwrap_or_else(|e| panic!("{}", e));

            for pair in pairs {
                // A pair is a combination of the rule which matched and a span of input
                let ident = MethodDef::parse(&pair);
                assert!(ident.is_some());

                let method = ident.unwrap();

                assert_eq!(method.access_modifier.to_string(), access_mod);
                assert_eq!(method.static_modifier, static_mod);
                assert_eq!(method.name.to_string(), fn_name);
            }
        }
    }

    #[test]
    fn test_grammer2() {
        let test_cases = vec!["private <K, T extends String> Map<K, A<T>> genMethod() {}"];

        for case in test_cases {
            let pairs = IdentParser::parse(Rule::method, case).unwrap_or_else(|e| panic!("{}", e));

            for pair in pairs {
                // A pair is a combination of the rule which matched and a span of input
                let ident = MethodDef::parse(&pair);
                assert!(ident.is_some());

                let method = ident.unwrap();
                println!("{}", method);
            }
        }
    }
}
