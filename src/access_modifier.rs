use std::fmt::Display;

pub enum AccessModifier {
    Public,
    Private,
    Protected,
    Default,
}

impl From<&str> for AccessModifier {
    fn from(am: &str) -> Self {
        match am {
            "public" => AccessModifier::Public,
            "private" => AccessModifier::Private,
            "protected" => AccessModifier::Protected,
            _ => AccessModifier::Default,
        }
    }
}

impl Display for AccessModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let access_str = match self {
            AccessModifier::Public => "public",
            AccessModifier::Private => "private",
            AccessModifier::Protected => "protected",
            AccessModifier::Default => "default",
        };

        write!(f, "{}", access_str)
    }
}
