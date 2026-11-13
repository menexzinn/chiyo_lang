use crate::utils::error::Error;
use crate::frontend::Stmt;

pub fn check_type(expect: String, found: Stmt) {
    let found_str = match found {
        Stmt::IntegerLiteral{ value: _ } => "int",
        Stmt::DecimalLiteral{ value: _ } => "float",
        Stmt::BooleanLiteral{ value: _ } => "bool",
        Stmt::StringLiteral{ value: _ } => "str",
        Stmt::NilLiteral => "nil",
        _ => unimplemented!(),
    };

    if expect != found_str {
        Error::Other.panic(format!("Expected {expect}, found {found_str}"));
    }
}
