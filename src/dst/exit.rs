use crate::frontend::Stmt;

pub fn exit(args: Vec<Box<Stmt>>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if args.len() > 1 {
        return result;
    }

    if let Stmt::IntegerLiteral{ value } = *args[0] {
        result.push("  mov rax, 60\n".to_string());
        result.push(format!("  mov rdi, {value}\n"));
        result.push("  syscall\n".to_string());
    }
    result
}
