use crate::frontend::{Stmt, BinaryOperator, UnaryOperator};

fn calculate(operator: BinaryOperator, a: f32, b: f32) -> f32 {
    match (operator, a, b) {
        (BinaryOperator::Add, _, 0.0) => a,
        (BinaryOperator::Add, 0.0, _) => b,
        (BinaryOperator::Subtract, _, 0.0) => a,
        (BinaryOperator::Subtract, 0.0, _) => -b,

        (BinaryOperator::Multiply, _, 0.0)
        | (BinaryOperator::Multiply, 0.0, _) => 0.0,

        (BinaryOperator::Add, _, _) => a + b,
        (BinaryOperator::Subtract, _, _) => a - b,
        (BinaryOperator::Multiply, _, _) => a * b,
        (BinaryOperator::Divide, _, _) => a / b,

        _ => unimplemented!(),
    }
}

fn operate_expr(operator: BinaryOperator, lhs: Stmt, rhs: Stmt) -> Stmt {
    match (&lhs, &rhs) {
        // operação direta
        (Stmt::IntegerLiteral{ value: a }, Stmt::IntegerLiteral { value: b }) => {
            match operator {
                BinaryOperator::Divide => Stmt::DecimalLiteral{
                    value: calculate(operator, *a as f32, *b as f32)
                },
                _ => {
                    let value = calculate(operator, *a as f32, *b as f32);
                    Stmt::IntegerLiteral{ value: value as i32 }
                },
            }
        },
        (Stmt::DecimalLiteral{ value: a }, Stmt::DecimalLiteral{ value: b }) => {
            let value = calculate(operator, *a, *b);
            Stmt::DecimalLiteral{ value }
        },

        // operação "cruzada"
        (Stmt::IntegerLiteral{ value: a }, Stmt::DecimalLiteral { value: b }) => {
            let value = calculate(operator, *a as f32, *b);
            Stmt::DecimalLiteral { value }
        },
        (Stmt::DecimalLiteral{ value: a }, Stmt::IntegerLiteral { value: b }) => {
            let value = calculate(operator, *a, *b as f32);
            Stmt::DecimalLiteral { value }
        },

        (Stmt::Identifier{ value: _ }, _)
        | (_, Stmt::Identifier{ value: _ })
        | (Stmt::BinaryExpression{ operator: _, lhs: _, rhs: _ }, _)
        | (_, Stmt::BinaryExpression{ operator: _, lhs: _, rhs: _ }) => Stmt::BinaryExpression {
            operator, lhs: Box::new(lhs), rhs: Box::new(rhs)
        },
        _ => unimplemented!(),
    }
}

fn optimize_bin_expr(bin_expr: Stmt) -> Stmt {
    let Stmt::BinaryExpression{ operator, lhs, rhs } = bin_expr else { unreachable!() };

    let opt_lhs = optimize_statements(*lhs);
    let opt_rhs = optimize_statements(*rhs);

    operate_expr(operator, opt_lhs, opt_rhs)
}

fn optimize_unary_expr(unary_expr: Stmt) -> Stmt {
    let Stmt::UnaryExpression{ operator, rhs } = unary_expr else { unreachable!() };

    let opt_rhs = optimize_statements(*rhs);

    match (&operator, &opt_rhs) {
        (UnaryOperator::MathematicalNegative, Stmt::IntegerLiteral{ value }) => Stmt::IntegerLiteral{ value: -value },
        (UnaryOperator::MathematicalNegative, Stmt::DecimalLiteral{ value }) => Stmt::DecimalLiteral{ value: -value },
        (UnaryOperator::LogicalNegation, Stmt::BooleanLiteral{ value }) => Stmt::BooleanLiteral{ value: !value },
        (_, Stmt::Identifier{ value: _ }) => Stmt::UnaryExpression{ operator, rhs: Box::new(opt_rhs) },
        _ => unimplemented!(),
    }
}

fn optimize_statements(stmt: Stmt) -> Stmt {
    match stmt {
        il @ Stmt::IntegerLiteral{ value: _ } => il,
        dl @ Stmt::DecimalLiteral{ value: _ } => dl,
        bl @ Stmt::BooleanLiteral{ value: _ } => bl,
        id @ Stmt::Identifier{ value: _ } => id,

        be @ Stmt::BinaryExpression{ operator: _, lhs: _, rhs: _ } => optimize_bin_expr(be),
        ue @ Stmt::UnaryExpression{ operator: _, rhs: _ } => optimize_unary_expr(ue),
        Stmt::VariableDeclaration{ name, var_type, value } => {
            let opt_value = optimize_statements(*value);
            Stmt::VariableDeclaration { name, var_type, value: Box::new(opt_value) }
        },
        _ => stmt,
    }
}

pub fn optimize(program: Stmt) -> Stmt {
    let Stmt::Program{ body } = program else { unreachable!() };
    Stmt::Program{
        body: body.into_iter().map(|stmt| Box::new(optimize_statements(*stmt))).collect()
    }
}
