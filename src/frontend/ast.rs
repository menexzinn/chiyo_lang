#[derive(Clone, Debug)]
pub enum UnaryOperator {
    MathematicalNegative,
    LogicalNegation,
}

#[derive(Clone, Debug)]
pub enum BinaryOperator {
    // arithmetic
    Add, Subtract, Multiply, Divide,

    // comparative
    Equals, NotEqual,
    GreaterEqual, LessEqual,
    Greater, Less,
    And, Or,
}

#[derive(Clone, Debug)]
pub struct FunctionArgument {
    pub arg_name: String,
    pub arg_type: String,
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Program {
        body: Vec<Box<Stmt>>,
    },

    StringLiteral { value: String },
    IntegerLiteral { value: i32 },
    DecimalLiteral { value: f32 },
    BooleanLiteral { value: bool },
    Identifier { value: String },
    NilLiteral,

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
    FunctionCall {
        name: String,
        args: Vec<Box<Stmt>>,
        is_dst: bool,
    },
}
