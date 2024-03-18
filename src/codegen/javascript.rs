use std::fmt::Display;

pub struct Variable {
    declaration: Declaration,
    typ: Type,
    name: String,
    value: String,
}

impl Variable {
    fn with_jsdoc(&self) -> String {
        format!(
            "{}\n{} {} = {};",
            self.typ.to_jsdoc(),
            self.declaration,
            self.name,
            self.value
        )
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} = {};", self.declaration, self.name, self.value)
    }
}

pub enum Type {
    String,
    Number,
    Bigint,
    Boolean,
    Undefined,
    Null,
    Symbol,
    Object,
}

impl Type {
    pub fn to_jsdoc(&self) -> String {
        format!("/** @type {{{self}}} */")
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::String => write!(f, "string"),
            Type::Number => write!(f, "number"),
            Type::Bigint => write!(f, "bigint"),
            Type::Boolean => write!(f, "boolean"),
            Type::Undefined => write!(f, "undefined"),
            Type::Null => write!(f, "null"),
            Type::Symbol => write!(f, "symbol"),
            Type::Object => write!(f, "object"),
        }
    }
}

pub enum Declaration {
    Const,
    Let,
    Var,
}

impl Display for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Declaration::Const => write!(f, "const"),
            Declaration::Let => write!(f, "let"),
            Declaration::Var => write!(f, "var"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::codegen::javascript::{Declaration, Type};

    use super::Variable;

    #[test]
    fn create_var() {
        let javascript = "/** @type {number} */\nlet xd = 3;";

        let var = Variable {
            declaration: Declaration::Let,
            typ: Type::Number,
            name: "xd".to_string(),
            value: "3".to_string(),
        };

        assert_eq!(javascript, var.with_jsdoc());
    }
}
