use std::fmt::Display;

use pest::iterators::Pair;

use crate::{
    access_modifier::AccessModifier, attribute::AttributeDef, ident::Ident, method::MethodDef,
    type_def::TypeDef, FromNode, Rule,
};

#[derive(Clone)]
pub struct ClassDef {
    pub name: Ident,
    pub extends: TypeDef,
    pub is_static: bool,
    pub implements: Vec<TypeDef>,
    pub access_modifier: AccessModifier,
    pub methods: Vec<MethodDef>,
    pub attributes: Vec<AttributeDef>,
    pub nested_classes: Vec<ClassDef>,
}

impl FromNode for ClassDef {
    fn parse(rule: &Pair<'_, Rule>) -> Option<Self> {
        match rule.as_rule() {
            Rule::class => {
                let mut cls = ClassDef::empty();
                for imp in rule.clone().into_inner() {
                    match imp.as_rule() {
                        Rule::access_modifier => {
                            cls.access_modifier = AccessModifier::from(imp.as_str());
                        }
                        Rule::ident => {
                            let ident = Ident::parse(&imp).unwrap();
                            cls.name = ident;
                        }
                        Rule::static_modifier => {
                            cls.is_static = true;
                        }
                        Rule::extends => {
                            for imp2 in imp.into_inner() {
                                match imp2.as_rule() {
                                    Rule::ty => cls.extends = TypeDef::parse(&imp2).unwrap(),
                                    _ => {}
                                }
                            }
                        }
                        Rule::implements => {
                            for imp2 in imp.into_inner() {
                                println!("{:?}", imp2.as_rule());
                                match imp2.as_rule() {
                                    Rule::ty => cls.implements.push(TypeDef::parse(&imp2).unwrap()),
                                    _ => {}
                                }
                            }
                        }
                        Rule::class_block => {
                            for imp2 in imp.into_inner() {
                                match imp2.as_rule() {
                                    Rule::method => {
                                        cls.methods.push(MethodDef::parse(&imp2).unwrap())
                                    }
                                    Rule::attribute => {
                                        cls.attributes.push(AttributeDef::parse(&imp2).unwrap())
                                    }
                                    Rule::class => {
                                        cls.nested_classes.push(ClassDef::parse(&imp2).unwrap())
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Some(cls)
            }
            _ => None,
        }
    }
}

impl Display for ClassDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let interfaces = self
            .implements
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let methods = self
            .methods
            .iter()
            .map(|i| i.to_string().replace("\n", "\n\t\t"))
            .map(|i| format!("\n\t\t{}", i))
            .collect::<Vec<_>>()
            .join(",");

        let attrs = self
            .attributes
            .iter()
            .map(|i| i.to_string().replace("\n", "\n\t\t"))
            .map(|i| format!("\n\t\t{}", i))
            .collect::<Vec<_>>()
            .join(",");

        let nested_classes = self
            .nested_classes
            .iter()
            .map(|i| i.to_string().replace("\n", "\n\t\t"))
            .map(|i| format!("\n\t\t{}", i))
            .collect::<Vec<_>>()
            .join(",");
        write!(
            f,
            "class {}
            access_modifier: {},
            is_static?: {}
            super_class: {},
            interfaces: {},
            attributes: {},
            methods: {},
            nested_classes: {}
        ",
            self.name,
            self.access_modifier,
            self.is_static,
            self.extends,
            interfaces,
            attrs,
            methods,
            nested_classes
        )
    }
}

impl ClassDef {
    fn empty() -> Self {
        Self {
            name: Ident::empty(),
            extends: TypeDef::empty(),
            implements: Vec::new(),
            access_modifier: AccessModifier::Default,
            methods: Vec::new(),
            attributes: Vec::new(),
            is_static: false,
            nested_classes: Vec::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use pest::Parser;

    use crate::{class::ClassDef, FromNode, IdentParser, Rule};

    #[test]
    fn test_grammer() {
        let test_cases = vec![
            "
            public class App {
                public static void main(String[] args) {
                    System.out.println(\"My First Java Program\");
            
                }
            };
            ",
            "
            class App {
                public static void main(String[] args) {
                    System.out.println(\"My First Java Program\");
            
                }
            };
            ",
            "
            // comment
            class App {
                public static void main(String[] args) {
                    System.out.println(\"My First Java Program\");
            
                }
            };
            ",
            "
            class App {
                public static void main(String[] args) {
                    System.out.println(\"My First Java Program\");
            
                }
                public String myMethod() {
                    return \"Hello\";
                }
            };
            ",
            "
            public class App extends MyApp2 implements I32, I43 {
                public static void main(String[] args) {
                    System.out.println(\"My First Java Program\");
            
                }
            };
            ",
            "
            public class App extends MyApp2 implements I32, I43 {
                public static final String SOMETHING = \"Hello\";

                public static void main(String[] args) {
                    System.out.println(\"My First Java Program\");
            
                }
            };
            ",
            "
            public class C1 {
                public static class C2 {}
            }
            ",
            "
            private final static class A
        extends B
    {

    }
            ",
        ];

        for case in test_cases {
            let pairs = IdentParser::parse(Rule::class, case).unwrap_or_else(|e| panic!("{}", e));

            for pair in pairs {
                // A pair is a combination of the rule which matched and a span of input
                let cls = ClassDef::parse(&pair);
                assert!(cls.is_some());

                println!("{}", cls.unwrap());
            }
        }
    }
}
