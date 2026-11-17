use crate::frontend::Stmt;

// TODO: accept evaluated binary expressions
pub fn exit(args: Vec<Stmt>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if args.len() > 1 {
        return result;
    }

    let arg = &args[0];
    match arg {
        Stmt::IntegerLiteral(value) => {
            result.push(format!("  mov rdi, {value}\n"));
            result.push("  call exit\n".to_string());
        }
        Stmt::FloatLiteral(value) => {
            let location = *value as i32;
            result.push(format!("  mov rdi, [rbp - {location}]\n"));
            result.push("  call exit\n".into());
        }
        _ => panic!("CODEGEN: Value for exit must have type int"),
    };
    result
}
