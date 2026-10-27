# Ladies and gentlemen
# Introducing the CHIYO lang!
# And the rust flavored compiler

Under development and has almost no functionality

## Syntax:
When defining variables you must define its type

```
let text: str = "America ya!"
dst->print(text)

let x: int = 23
let y: int = 23 * 3 - 2
dst->exit(y)
```

Control flow is unsuported for now, but it's going to be like
```
if (condition) {
  // code
} else {
  // code
}

// array unsuported for now, same for other data structures
for i in array {
  // insert code
}
```

## Notes:
As you could see, "dst" is like rust's "std", or more like "sti"... And arrow operator is coming to substitute dots!!!
Also, I'm unable to directly print strings for now, so you just define a variable to hold the string and then you can print it!

Assembly is such a pain to handle but this project is being really fun to make without llvm or smth like that
