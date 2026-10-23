use crate::frontend::{Stmt, BinaryOperator, UnaryOperator};
use std::collections::HashMap;

pub struct Optimizer {
    opt_vars: HashMap<String, Box<Stmt>>,
}

impl Optimizer {
    pub fn new() -> Self {
        Self{ opt_vars: HashMap::new() }
    }

    fn insert_var(&mut self, key: String, value: Box<Stmt>) {
        self.opt_vars.insert(key, value);
    }
    fn get_var(&self, key: &String) -> Option<&Box<Stmt>> {
        self.opt_vars.get(key)
    }

    // arithmetic optimization
    fn calculate(&self, operator: BinaryOperator, a: f32, b: f32) -> f32 {
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

    // constant folding
    fn operate_expr(&self, operator: BinaryOperator, lhs: Stmt, rhs: Stmt) -> Stmt {
        match (&lhs, &rhs) {
            (Stmt::IntegerLiteral{ value: a }, Stmt::IntegerLiteral { value: b }) => match operator {
                BinaryOperator::Divide => Stmt::DecimalLiteral{
                    value: self.calculate(operator, *a as f32, *b as f32)
                },
                _ => {
                    let value = self.calculate(operator, *a as f32, *b as f32);
                    Stmt::IntegerLiteral{ value: value as i32 }
                },
            },
            (Stmt::DecimalLiteral{ value: a }, Stmt::DecimalLiteral{ value: b }) => {
                let value = self.calculate(operator, *a, *b);
                Stmt::DecimalLiteral{ value }
            },

            (Stmt::IntegerLiteral{ value: a }, Stmt::DecimalLiteral { value: b }) => {
                let value = self.calculate(operator, *a as f32, *b);
                Stmt::DecimalLiteral { value }
            },
            (Stmt::DecimalLiteral{ value: a }, Stmt::IntegerLiteral { value: b }) => {
                let value = self.calculate(operator, *a, *b as f32);
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

    fn optimize_bin_expr(&mut self, bin_expr: Stmt) -> Stmt {
        let Stmt::BinaryExpression{ operator, lhs, rhs } = bin_expr else { unreachable!() };

        let opt_lhs = self.optimize_statements(*lhs);
        let opt_rhs = self.optimize_statements(*rhs);

        self.operate_expr(operator, opt_lhs, opt_rhs)
    }

    fn optimize_unary_expr(&mut self, unary_expr: Stmt) -> Stmt {
        let Stmt::UnaryExpression{ operator, rhs } = unary_expr else { unreachable!() };

        let opt_rhs = self.optimize_statements(*rhs);

        match (&operator, &opt_rhs) {
            (UnaryOperator::MathematicalNegative, Stmt::IntegerLiteral{ value }) => Stmt::IntegerLiteral{ value: -value },
            (UnaryOperator::MathematicalNegative, Stmt::DecimalLiteral{ value }) => Stmt::DecimalLiteral{ value: -value },
            (UnaryOperator::LogicalNegation, Stmt::BooleanLiteral{ value }) => Stmt::BooleanLiteral{ value: !value },
            (_, Stmt::Identifier{ value: _ }) => Stmt::UnaryExpression{ operator, rhs: Box::new(opt_rhs) },
            _ => unimplemented!(),
        }
    }

    fn optimize_statements(&mut self, stmt: Stmt) -> Stmt {
        match stmt {
            il @ Stmt::IntegerLiteral{ value: _ } => il,
            dl @ Stmt::DecimalLiteral{ value: _ } => dl,
            bl @ Stmt::BooleanLiteral{ value: _ } => bl,

            // constant propagation
            Stmt::Identifier{ value } => {
                // fuck shared references, fuck clone, fuck rust, fuck everything
                match self.get_var(&value) {
                    Some(s) => self.optimize_statements(*s.clone()),
                    None => Stmt::Identifier{ value }, // throw error later cause ts not defined
                }
            },

            be @ Stmt::BinaryExpression{ operator: _, lhs: _, rhs: _ } => self.optimize_bin_expr(be),
            ue @ Stmt::UnaryExpression{ operator: _, rhs: _ } => self.optimize_unary_expr(ue),
            Stmt::VariableDeclaration{ name, var_type, value } => {
                let opt_value = self.optimize_statements(*value);
                match opt_value {
                    // love gambiarras
                    Stmt::IntegerLiteral{ value } => {
                        self.insert_var(name.clone(), Box::new(Stmt::IntegerLiteral{ value }))
                    },
                    Stmt::DecimalLiteral{ value } => {
                        self.insert_var(name.clone(), Box::new(Stmt::DecimalLiteral{ value }))
                    },
                    _ => ()
                }
                Stmt::VariableDeclaration { name, var_type, value: Box::new(opt_value) }
            },
            Stmt::FunctionCall{ name, args } => {
                Stmt::FunctionCall{
                    name,
                    args: args
                        .into_iter()
                        .map(|arg| Box::new(self.optimize_statements(*arg)))
                        .collect()
                }
            },
            _ => stmt,
        }
    }

    pub fn optimize(&mut self, program: Stmt) -> Stmt {
        let Stmt::Program{ body } = program else { unreachable!() };

        Stmt::Program{
            body: body
                .into_iter()
                .map(|stmt| Box::new(self.optimize_statements(*stmt)))
                .collect()
        }
    }
}
