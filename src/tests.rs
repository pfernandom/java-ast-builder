#[cfg(test)]
mod test {
    use pest::Parser;

    use crate::{ident::Ident, FromNode, IdentParser, Rule};

    #[test]
    fn method_grammar() {
        let pairs = IdentParser::parse(
            Rule::method,
            "public static void main(List<String> args) { System.out.println(\"Hello world\"); }",
        )
        .unwrap_or_else(|e| panic!("{}", e));

        for pair in pairs {
            // A pair is a combination of the rule which matched and a span of input
            println!("Rule:    {:?}", pair.as_rule());
            println!("Span:    {:?}", pair.as_span());
            println!("Text:    {}", pair.as_str());

            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::ty => {
                        let tty = inner_pair.as_str();
                        println!("{}", tty)
                    }
                    _r => {
                        let tty = inner_pair.as_str();
                        println!("{}", tty)
                    }
                }
            }
        }
    }

    #[test]
    fn class_grammar() {
        let code = "
    class MyClass {}
    ";

        let pairs = IdentParser::parse(Rule::class, code).unwrap_or_else(|e| panic!("{}", e));

        for pair in pairs {
            // A pair is a combination of the rule which matched and a span of input
            let ident = Ident::parse(&pair).unwrap();
            println!("{}", ident)
        }
    }
}
