default: run

run:
	cargo run -- main.chy
	nasm -f elf64 main.asm -o main.o
	nasm -f elf64 src/backend/runtime.asm -o runtime.o
	ld main.o runtime.o -o main
	./main

clean:
	rm -rf main main.o main.asm runtime.o
