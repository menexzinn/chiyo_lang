use std::process::Command;

pub fn assemble() {
    Command::new("nasm")
        .args(["-f", "elf64", "-o", "main.o", "main.asm"])
        .output()
        .expect("ERROR: Could not generate object with 'nasm' command");
}

pub fn link() {
    Command::new("ld")
        .args(["-o", "main", "main.o"])
        .output()
        .expect("ERROR: Could not generate executable with 'ld' command");
}
