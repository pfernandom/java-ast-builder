use std::fmt::Display;

use pest::iterators::Pair;

use crate::{ident::Ident, FromNode, Rule};

#[derive(Clone)]
pub struct TypeDef {
    name: Ident,
    generic_params: Vec<TypeDef>,
    array: bool,
}

impl FromNode for TypeDef {
    fn parse(rule: &Pair<'_, Rule>) -> Option<Self> {
        match rule.as_rule() {
            Rule::ty => {
                let mut ty = TypeDef::empty();
                for inner in rule.clone().into_inner() {
                    println!("inner={:?}", inner.as_rule());
                    match inner.as_rule() {
                        Rule::generic_params => {
                            for inner3 in inner.into_inner() {
                                match inner3.as_rule() {
                                    Rule::ty => {
                                        let gen_ty = TypeDef::parse(&inner3).unwrap();
                                        // println!("gen_ty={}", gen_ty);
                                        ty.generic_params.push(gen_ty)
                                    }
                                    _ => {}
                                }
                            }
                        }

                        Rule::object => ty.name = Ident::new(inner.as_str()),
                        Rule::ident => ty.name = Ident::parse(&inner).unwrap(),

                        Rule::dot => ty.name = Ident::parse(&inner).unwrap(),
                        _ => {}
                    }
                }
                Some(ty)
            }
            _ => None,
        }
    }
}

impl Display for TypeDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl TypeDef {
    pub fn empty() -> Self {
        Self {
            name: Ident::empty(),
            generic_params: Vec::new(),
            array: false,
        }
    }
}

#[cfg(test)]
mod test {
    use pest::Parser;

    use crate::{FromNode, IdentParser, Rule};

    use super::TypeDef;

    #[test]
    fn test_grammer() {
        let test_cases = vec!["String", "com.something.MyClass"];

        for case in test_cases {
            let pairs = IdentParser::parse(Rule::ty, case).unwrap_or_else(|e| panic!("{}", e));

            for pair in pairs {
                // A pair is a combination of the rule which matched and a span of input

                // println!("{:?}", pair.as_rule());
                let package = TypeDef::parse(&pair);
                assert!(package.is_some());
                println!("{}", package.unwrap().to_string())
            }
        }
    }

    #[test]
    fn test_generic() {
        let test_cases = vec!["List<String>", "com.something.MyClass<String>"];

        for case in test_cases {
            let pairs = IdentParser::parse(Rule::ty, case).unwrap_or_else(|e| panic!("{}", e));

            for pair in pairs {
                // A pair is a combination of the rule which matched and a span of input
                let maybe_ty = TypeDef::parse(&pair);
                assert!(maybe_ty.is_some());
                let ty = maybe_ty.unwrap();

                println!("{}", ty);

                assert!(
                    ty.name.to_string() == "List" || ty.name.to_string() == "com.something.MyClass"
                );
                assert!(ty
                    .generic_params
                    .iter()
                    .all(|p| p.name.to_string() == "String"));
            }
        }
    }

    #[test]
    fn test_generic_2() {
        let test_cases = vec!["Map<String,Boolean>"];

        for case in test_cases {
            let pairs = IdentParser::parse(Rule::ty, case).unwrap_or_else(|e| panic!("{}", e));

            for pair in pairs {
                // A pair is a combination of the rule which matched and a span of input
                let maybe_ty = TypeDef::parse(&pair);
                assert!(maybe_ty.is_some());
                let ty = maybe_ty.unwrap();

                assert_eq!(ty.name.to_string(), "Map");
                assert_eq!(ty.generic_params.len(), 2);

                assert_eq!(ty.generic_params.get(0).unwrap().to_string(), "String");
                assert_eq!(ty.generic_params.get(1).unwrap().to_string(), "Boolean");
            }
        }
    }

    #[test]
    fn test_generic_nested() {
        let test_cases = vec!["Map<String,List<Boolean>>"];

        for case in test_cases {
            let pairs = IdentParser::parse(Rule::ty, case).unwrap_or_else(|e| panic!("{}", e));

            for pair in pairs {
                // A pair is a combination of the rule which matched and a span of input
                let maybe_ty = TypeDef::parse(&pair);
                assert!(maybe_ty.is_some());
                let ty = maybe_ty.unwrap();

                assert_eq!(ty.name.to_string(), "Map");
                assert_eq!(ty.generic_params.len(), 2);

                assert_eq!(ty.generic_params.get(0).unwrap().to_string(), "String");
                let second_gen = ty.generic_params.get(1).unwrap();
                assert_eq!(second_gen.to_string(), "List");
                assert_eq!(second_gen.generic_params.len(), 1);
                assert_eq!(
                    second_gen.generic_params.get(0).unwrap().to_string(),
                    "Boolean"
                );
            }
        }
    }
}
