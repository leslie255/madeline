<img alt="MADELINE" src="https://i.imgur.com/CU7gR2X.png" style="height:50px;">

**A less professional version of LLVM (WIP)**

I'm planning to use this project as the backend of [leslie255/sharklang](https://github.com/leslie255/sharklang).

<img alt="language: rust" src="https://i.imgur.com/Mae21iF.png" style="height:24px;">

## What does it do?
Like LLVM, Madeline is a compiler backend, it takes in some IR - Intermedian Representation code, a type of code that is more abstract than machine code, but more basic than higher level programming languages like C, and generates a compiled program from that.
The current features of Madeline is pretty limited, it can only generates x86_64 NASM assembly in macho64 and elf64 format.

## Using the program
*** Because this project is still in its early stage, this documentation may not be up to date***

To generate assembly code, use this command:
``` Bash
$ madeline source.mir elf64 output.asm
```
To generate for macOS, replace `elf64` with `macho64`

## Basic Syntax:

### Instructions

An instruction in MIR is consisted of an opcode, an operand and a second optional operand, this is an example of an instruction:

```
set_var    var i32 x    data i32 255
```

In this case, `set_var` is an opcode, `var i32 x` is the first operand, and `data i32 255` is the second operand.

An operand is consisted of three parts, operand type, data type, and content. In the example above, the operand type of `var i32 x` is `var`, the data type is `i32` and the content is `x`.


For some types of operands, one of these thee fields may be irrelevant, such as in this example below:
```
fn_call    fn _ test_func	_
```
The operand type `fn`, which is basically a function label, doesn't need to have a data type, therefore it is left blank by the underscore symbol `_`. Similarly, in cases where the content field is irrelevant, it may also be left blank by `_`. This instruction also doesn't have the second operand, and is therefore also left irrelevant by the underscore symbol `_`.


### Functions
Obviously Madeline doesn't allow instructions at top level, everything has to be inside a function. A function can be defined like this:

```
/ btw comments are marked like this
#fn_def main
    a u64
    b i32
{
    var_set     var u64 a       data u64 255
    ret_val     data i32 0		_
	/ also has a `ret_void` opcode for no return values
	/ both two operands have to be left irrelevant for `ret_void`
}
```

In MIR, variables are defined at the start of every code block as shown above, this is due to the way that assembly instructions typically works.

Note that all top level elements, such as `#fn_def`, are marked with the `#` symbol, this doesn't really help with the parser in any ways, but it does help to make the code look cleaner on the eye.

Also note that all tokens have to be separated by spaces so things like `main{` would be treated as a single token `main{` instead of `main` and `{` as two separated tokens, this *is* to make the parser easier to write and faster to run, after all, Madeline is a compiler backend, so the parser part doesn't need to be very advanced.


### Function arguments and return values
Unlike higher level languages, there is no safety check for the number of arguments or the type of the arguments, Madeline assumes that the compiler at frontend has already done such checks.

In the body of a function, use the `arg` operand to get the arguments. To call a function with arguments, use the `set_arg` instruction.

To get the return value of the previously called function, use the `result` operand.

```
#fn_def test_func
    num i32
{
    / 0 means the first argument
    var_set     var i32 num     arg i32 0
    ret_val     var i32 num		_
}

#fn_def main
    a i32
{
    set_arg     arg i32 0       data i32 255
    fn_call     fn _ test_func	_
    var_set     var i32 a       result i32 _
    ret_val     var i32 a		_
}
```

## Building
Madeline do not use any third-party library, you can build the project by:
``` Bash
$ cargo build --release
# the output executable is at `target/release/madeline`
```
