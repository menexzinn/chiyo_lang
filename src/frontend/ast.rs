#[derive(Debug)]
pub enum UnaryOperator {
    MathematicalNegative,
    LogicalNegation,
}

#[derive(Debug)]
pub enum BinaryOperator {
    // arithmetic
    Add, Subtract, Multiply, Divide,

    // comparative
    Equals, NotEqual,
    GreaterEqual, LessEqual,
    Greater, Less,
    And, Or,
}

#[derive(Debug)]
pub struct FunctionArgument {
    pub arg_name: String,
    pub arg_type: String,
}

#[derive(Debug)]
pub enum Stmt {
    Program {
        body: Vec<Box<Stmt>>,
    },

    NumericLiteral {
        value: u32,
    },
    BooleanLiteral {
        value: bool,
    },
    Identifier {
        value: String,
    },
    NullLiteral,

    UnaryExpression {
        operator: UnaryOperator,
        rhs: Box<Stmt>,
    },
    BinaryExpression {
        operator: BinaryOperator,
        lhs: Box<Stmt>,
        rhs: Box<Stmt>,
    },

    VariableDeclaration {
        is_constant: bool,
        name: String,
        var_type: String,
        value: Box<Stmt>,
    },
    FunctionDeclaration {
        name: String,
        return_type: Option<String>,
        args: Vec<FunctionArgument>,
        body: Vec<Box<Stmt>>,
    },
}

// before complaining at me, I am going to ensure that the value of the variable is an
// expression which will be then evaluated as a literal, either a number or a string
