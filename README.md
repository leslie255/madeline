# Madeline
**A less professional version of LLVM (WIP)**

I'm planning to use this project as the backend of [leslie255/sharklang](https://github.com/leslie255/sharklang).

### Features
Madeline can currently generate x86_64 NASM assembly code in elf64 and macho64 format. And there are no optimizations what-so-ever.

### Using the program
To generate assembly code, use this command:
``` Bash
$ madeline source.mir elf64 output.asm
```
To generate for macOS, replace `elf64` with `macho64`

## Basic Syntax:

### Instructions

An instruction in MIR is consisted of an opcode, an operand and a second optional operand, this is an example of an instruction:

```
set_var    var i32 x    data i32 255    ;
```

In this case, `set_var` is an opcode, `var i32 x` is the first operand, and `data i32 255` is the second operand.

An operand is consisted of three parts, operand type, data type, and content. In the example above, the operand type of `var i32 x` is `var`, the data type is `i32` and the content is `x`.

Note that in MIR, all tokens have to be separated by spaces, which means that the semicolon at the end of each line of instructions cannot be connected to the last token;

For some types of operands, one of these thee fields may be irrelevent, such as in this example below:
```
fn_call    fn _ test_func    ;
```
The operand type `fn`, which is basically a function label, doesn't need to have a data type, therefore it is left blank by the underscore symbol `_`. Similarly, in cases where the content field is irrelevant, it may also be left blank by `_`. The above instruction is also an example of an instruction with only one operand.

### Functions
Obviously Madeline doesn't allow instructions at top level, everything has to be inside a function. A function can be defined like this:

```
/ btw comments are marked like this
#fn_def main
    a u64
    b i32
{
    var_set     var u64 a       data u64 255    ;
    ret         data i32 0                      ;
}
```

Note that all top level elements, such as `#fn_def`, are marked with the `#` symbol just to make it look cleaner.

In MIR, variables are defined at the start of every code block as shown above, this is due to the way that assembly instructions typically works.

### Function arguments and return values
Unlike higher level languages, there is no safety check for the number of arugments, Madeline assumes that the compiler at frontend has already done such checks

In the implementation of a function, use the `arg` operand to get the arguments. To call a function with arguments, use the `set_arg` instruction.

To get the return value of the previously called function, use the `ret_val` operand.

```
#fn_def test_func
    num i32
{
    / 0 means the first argument
    var_set     var i32 num     arg i32 0       ;
    ret         var i32 num                     ;
}

#fn_def main
    a i32
{
    set_arg     arg i32 0       data i32 255    ;
    fn_call     fn _ test_func                  ;
    var_set     var i32 a       ret_val i32 _   ;
    ret         var i32 a                       ;
}
```

## Building
Madeline do not use any third-party library, you can build the project by:
``` Bash
$ cargo build --release
# the output executable is at `target/release/madeline`
```
