use crate::frontend::Stmt;

pub fn exit(args: Vec<Box<Stmt>>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if args.len() > 1 {
        return result;
    }

    match *args[0] {
        Stmt::IntegerLiteral{ value } => {
            result.push(format!("  mov rdi, {value}\n"));
            result.push("  call exit\n".to_string());
            result.push("  xor rdi, rdi\n".to_string());
        },
        _ => panic!("CODEGEN: Value for exit must have type int"),
    };
    result
}
