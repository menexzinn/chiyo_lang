section .text
  global exit
  global print

exit:
  mov rax, 60
  ; should move value to rdi!
  syscall
  ret

print:
  mov rax, 1
  mov rdi, 1
  ; move string to rsi and length to rdx
  syscall
  xor rax, rax
  xor rdi, rdi
  xor rsi, rsi
  xor rdx, rdx
  ret
