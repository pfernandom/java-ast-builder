use std::fmt::Display;

use pest::iterators::Pair;

use crate::{access_modifier::AccessModifier, ident::Ident, type_def::TypeDef, FromNode, Rule};

#[derive(Clone)]
pub struct AttributeDef {
    name: Ident,
    ty: TypeDef,
    value: String,
    static_modifier: bool,
    access_modifier: AccessModifier,
}

impl FromNode for AttributeDef {
    fn parse(rule: &Pair<'_, Rule>) -> Option<Self> {
        match rule.as_rule() {
            Rule::attribute => {
                let mut attr = AttributeDef::empty();

                for inner in rule.clone().into_inner() {
                    println!("{:?}", inner.as_rule());
                    match inner.as_rule() {
                        Rule::access_modifier => {
                            attr.access_modifier = AccessModifier::from(inner.as_str());
                        }
                        Rule::ty => attr.ty = TypeDef::parse(&inner).unwrap(),
                        Rule::assign_stmt => {
                            for inner2 in inner.clone().into_inner() {
                                match inner2.as_rule() {
                                    Rule::object => attr.name = Ident::new(&inner2.as_str()),
                                    Rule::exp => attr.value = (&inner2.as_str()).to_string(),
                                    _ => {}
                                }
                            }
                        }
                        Rule::static_modifier => attr.static_modifier = true,
                        // Rule::args => method.static_modifier = true,
                        _ => {}
                    }
                }

                Some(attr)
            }
            _ => None,
        }
    }
}

impl Display for AttributeDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},
            type: {},
            access_modifier: {},
            value: {},",
            self.name, self.ty, self.access_modifier, self.value
        )
    }
}

impl AttributeDef {
    pub fn empty() -> Self {
        Self {
            name: Ident::empty(),
            ty: TypeDef::empty(),
            value: String::from(""),
            access_modifier: AccessModifier::Default,
            static_modifier: false,
        }
    }
}

#[cfg(test)]
mod test {
    use pest::Parser;

    use crate::{attribute::AttributeDef, FromNode, IdentParser, Rule};

    #[test]
    fn test_grammer() {
        let test_cases = vec![
            (
                "public static final String SOMETHING = \"Hello\";",
                "public",
                true,
                "SOMETHING ",
            ),
            (
                "private final static ActorSummary.Fields _fields = new ActorSummary.Fields();",
                "private",
                true,
                "_fields ",
            ),
        ];

        for (case, access_mod, static_mod, fn_name) in test_cases {
            let pairs =
                IdentParser::parse(Rule::attribute, case).unwrap_or_else(|e| panic!("{}", e));

            for pair in pairs {
                // A pair is a combination of the rule which matched and a span of input
                let ident = AttributeDef::parse(&pair);
                assert!(ident.is_some());

                let attr = ident.unwrap();
                println!("{}", attr);

                assert_eq!(attr.access_modifier.to_string(), access_mod);
                assert_eq!(attr.static_modifier, static_mod);
                assert_eq!(attr.name.to_string(), fn_name);
            }
        }
    }

    #[test]
    fn test_grammer2() {
        let test_cases = vec!["public static final String SOMETHING = \"Hell\no\";"];

        for case in test_cases {
            let pairs =
                IdentParser::parse(Rule::attribute, case).unwrap_or_else(|e| panic!("{}", e));

            for pair in pairs {
                // A pair is a combination of the rule which matched and a span of input

                let ident = AttributeDef::parse(&pair);
                assert!(ident.is_some());

                let attr = ident.unwrap();
                println!("{}", attr);
            }
        }
    }
}
