use std::fmt::Display;

use pest::iterators::Pair;

use crate::{class::ClassDef, import::Import, package::PackageDef, FromNode, Rule};

pub struct FileDef {
    package: PackageDef,
    imports: Vec<Import>,
    classes: Vec<ClassDef>,
}

impl FromNode for FileDef {
    fn parse(rule: &Pair<'_, Rule>) -> Option<Self> {
        match rule.as_rule() {
            Rule::file => {
                let mut f = FileDef::empty();
                for inner in rule.clone().into_inner() {
                    match inner.as_rule() {
                        Rule::package => f.package = PackageDef::parse(&inner).unwrap(),
                        Rule::imports => {
                            for inner2 in inner.into_inner() {
                                match inner2.as_rule() {
                                    Rule::import => f.imports.push(Import::parse(&inner2).unwrap()),
                                    _ => {}
                                }
                            }
                        }
                        Rule::class => f.classes.push(ClassDef::parse(&inner).unwrap()),
                        _ => {}
                    }
                }

                Some(f)
            }
            _ => None,
        }
    }
}

impl Display for FileDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let imports = self
            .imports
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let classes = self
            .classes
            .iter()
            .map(|c| c.name.to_string())
            .collect::<Vec<_>>()
            .join(",");

        write!(
            f,
            "
        package:{}
        imports: {},
        classes: {}",
            self.package, imports, classes
        )
    }
}

impl FileDef {
    pub fn empty() -> Self {
        Self {
            package: PackageDef::empty(),
            imports: Vec::new(),
            classes: Vec::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use pest::Parser;

    use crate::{FromNode, IdentParser, Rule};

    use super::FileDef;

    #[test]
    fn test_grammer() {
        let test_cases = vec![
            "
        package com.app;

        import com.app.MyApp;
        import com.app.MyApp2;
        
        public class App extends MyApp2 implements I32, I43 {
            public static void main(String[] args) {
                System.out.println(\"My First Java Program\");
        
            }
        };
        ",
            "
        package com.app;

        import com.app.MyApp;
        import com.app.MyApp2;

        private static class MyClass {};
        ",
            "
        package com.app;

        import com.app.MyApp;
        import com.app.MyApp2;
        
        public class App extends MyApp2 implements I32, I43 {
            public static void main(String[] args) {
                System.out.println(\"My First Java Program\");
        
            }
        };

        private static class MyClass {};
        ",
        ];

        for case in test_cases {
            let pairs = IdentParser::parse(Rule::file, case).unwrap_or_else(|e| panic!("{}", e));

            for pair in pairs {
                // A pair is a combination of the rule which matched and a span of input
                let file = FileDef::parse(&pair);
                assert!(file.is_some());

                println!("{}", file.unwrap());
            }
        }
    }
}
