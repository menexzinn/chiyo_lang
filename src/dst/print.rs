use crate::frontend::Stmt;

pub fn print(args: Vec<Box<Stmt>>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if args.len() > 1 {
        return result;
    }

    // you print variables, not pure strings
    let variable = match &*args[0] {
        Stmt::Identifier { value } => value,
        _ => unimplemented!(),
    };

    result.push(format!("  mov rsi, {variable}\n")); // message
    result.push(format!("  mov rdx, {variable}Len\n")); // bytes
    result.push("  call print\n".to_string()); // call

    result
}
