section .text
  global exit
  global print

exit:
  mov rax, 60
  ; should move value to rdi!
  syscall
  ret

print:
  ; TODO: add real printing here!
  ret
