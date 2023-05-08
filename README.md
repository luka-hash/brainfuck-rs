# Brainfuck

A Brainfuck interpreter written in the Rust programming language, created for a presentation I gave about Esoteric programming languages.

### Implementation details
- tape consists of 30000 `u8` cells
- cell values wrap
- tape pointer wraps
- it creates a jump table to speed up jumping; this also means that it will not attempt to execute invalid programs (in terms of mismatched parentheses)

This implementation may not pass all tests, but it is capable of running non-trivial programs (non-trivial for Brainfuck, though) without errors. Additionally, it is small and simple enough that you can easily fix or adapt it to your needs without much trouble.

### TODO
- implement `#` for debugging purposes
- add additional informations for debugging, more specifically, when creating the jump table, return the location of error
- implement a flag for
  - buffered output on/off
  - logging levels (verbosity)

### Running
Simply use `cargo run` and pass the filenames of the programs you want to execute.

### License

MIT License :)

Regarding the example Brainfuck programs: If there is no license or credits inside the file itself, it means that it was either written by yours truly, or downloaded from http://brainfuck.org.
If you are the author of any of the presented example programs, and you want it removed, please write me an email at idontcare@ljudi.org.
