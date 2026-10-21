use crate::frontend::Stmt;

pub fn print(args: Vec<Box<Stmt>>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if args.len() > 1 {
        return result;
    }

    // rsi: String, rdx: number
    let (rsi, rdx) = match &*args[0] {
        Stmt::StringLiteral{ value } => (value.clone(), value.len()),
        Stmt::IntegerLiteral{ value } => (format!("{value}"), 4),
        Stmt::DecimalLiteral{ value } => (format!("{value}"), 4),
        _ => unimplemented!(),
    };

    result.push("  mov rax, 1\n".to_string()); // write
    result.push("  mov rdi, 1\n".to_string()); // stdout
    result.push(format!("  mov rsi, {rsi}\n")); // message
    result.push(format!("  mov rdx, {rdx}\n")); // bytes
    result.push("  syscall\n".to_string()); // call
    result
}
