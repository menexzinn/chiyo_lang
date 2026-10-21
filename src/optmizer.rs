use crate::frontend::{Stmt, BinaryOperator};

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
        (Stmt::IntegerLiteral { value: a }, Stmt::IntegerLiteral { value: b }) => {
            // let value = calculate(operator, a as f32, b as f32);
            // if let BinaryOperator::Divide = operator {
                // return Stmt::DecimalLiteral { value };
            // }
            // Stmt::IntegerLiteral { value: value as i32 }

            match operator {
                BinaryOperator::Divide => Stmt::DecimalLiteral {
                    value: calculate(operator, *a as f32, *b as f32)
                },
                _ => {
                    let value = calculate(operator, *a as f32, *b as f32);
                    Stmt::IntegerLiteral { value: value as i32 }
                },
            }
        },
        (Stmt::DecimalLiteral { value: a }, Stmt::DecimalLiteral { value: b }) => {
            let value = calculate(operator, *a, *b);
            Stmt::DecimalLiteral { value }
        },

        // operação "cruzada"
        (Stmt::IntegerLiteral { value: a }, Stmt::DecimalLiteral { value: b }) => {
            let value = calculate(operator, *a as f32, *b);
            Stmt::DecimalLiteral { value }
        },
        (Stmt::DecimalLiteral { value: a }, Stmt::IntegerLiteral { value: b }) => {
            let value = calculate(operator, *a, *b as f32);
            Stmt::DecimalLiteral { value }
        },

        (Stmt::Identifier{ value: _ }, _)
        | (_, Stmt::Identifier{ value: _ }) => Stmt::BinaryExpression {
            operator, lhs: Box::new(lhs), rhs: Box::new(rhs)
        },
        _ => unimplemented!(),
    }
}

fn optmize_bin_expr(bin_expr: Stmt) -> Stmt {
    let Stmt::BinaryExpression { operator, lhs, rhs } = bin_expr else { unreachable!() };

    let opt_lhs = match *lhs {
        b @ Stmt::BinaryExpression { operator: _, lhs: _, rhs: _ } => optmize_bin_expr(b),
        _ => *lhs,
    };

    let opt_rhs = match *rhs {
        b @ Stmt::BinaryExpression{ operator: _, lhs: _, rhs: _ } => optmize_bin_expr(b),
        _ => *rhs,
    };

    operate_expr(operator, opt_lhs, opt_rhs)
}

fn optmize_statements(stmt: Stmt) -> Stmt {
    match stmt {
        il @ Stmt::IntegerLiteral { value: _ } => il,
        dl @ Stmt::DecimalLiteral { value: _ } => dl,
        id @ Stmt::Identifier { value: _ } => id,
        be @ Stmt::BinaryExpression { operator: _, lhs: _, rhs: _ } => optmize_bin_expr(be),
        Stmt::VariableDeclaration { name, var_type, value } => {
            let opt_value = optmize_statements(*value);
            Stmt::VariableDeclaration { name, var_type, value: Box::new(opt_value) }
        },
        _ => unimplemented!(),
    }
}

pub fn optmize(program: Stmt) -> Stmt {
    let mut optimized_body: Vec<Box<Stmt>> = Vec::new();

    let original_body = match program {
        Stmt::Program { body } => body,
        _ => unreachable!(),
    };

    for stmt in original_body {
        optimized_body.push(Box::new(optmize_statements(*stmt)));
    }

    Stmt::Program { body: optimized_body }
}
