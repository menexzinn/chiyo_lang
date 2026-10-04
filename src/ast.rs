#[derive(Debug)]
pub enum Stmt {
    Program {
        body: Vec<Box<Stmt>>,
    },
    NumericLiteral {
        value: u32,
    },
    Identifier {
        value: String,
    },
    BinaryExpr {
        operator: char,
        lhs: Box<Stmt>,
        rhs: Box<Stmt>,
    },
    VariableDeclaration {
        name: String,
        value: Box<Stmt>,
        is_constant: bool,
    },
}

// before complaining at me, I am going to ensure that the value of the variable is an
// expression which will be then evaluated as a literal, either a number or a string
