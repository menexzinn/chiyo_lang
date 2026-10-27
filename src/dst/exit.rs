use crate::frontend::Stmt;

pub fn exit(args: Vec<Box<Stmt>>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if args.len() > 1 {
        return result;
    }

    let arg = args[0].clone();
    match *arg {
        Stmt::IntegerLiteral { value } => {
            result.push(format!("  mov rdi, {value}\n"));
            result.push("  call exit\n".to_string());
        }
        Stmt::Identifier { value } => {
            result.push(format!("  mov rdi, [rel {value}]\n"));
            result.push("  call exit\n".into());
        }
        _ => panic!("CODEGEN: Value for exit must have type int"),
    };
    result
}
