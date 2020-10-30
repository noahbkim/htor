# htor

`htor` is a commandline utility that allows you to conveniently generate binary payloads.
This simplest way to specify the contents of a payload is by using hex, decimal, binary, or string literals.
The values parsed in each line are directly appended to the resultant payload in the order they appear.
`htor` also offers several macros.
Currently supported are `@repeat`, `@define`, and `@assembly`.
Consider the following script included in examples:

```
# Output FF in binary, decimal, and hex
0b11111111 0d255 0xff ff

# Output 0x42 left-padded to a width of 4 bytes
[4]42

# Output 0d420 right-padded to a width of 8 bytes
0d420[0b1000]

# Define a new macro that takes one argument: name
@define greet name
  "Hi, " $name

# Define a macro that takes no arguments
@define steak
  DEADBEEF

# Repeat the contents of this block two times
@repeat 2
  $greet($steak)

# Output $steak with reverse endianness, then normal
< $steak > $steak

# Any normal expression may be placed in an argument
$greet(< 02 01 > 03 04)
$greet(< $greet())
< $greet($greet)
```

If we build an run `htor -d examples/basic.txt`, we can see the debug hex of the output we'd normally get:

```
FF FF FF FF 00 00 00 42  01 A4 00 00 00 00 00 00
48 69 2C 20 DE AD BE EF  48 69 2C 20 DE AD BE EF
EF BE AD DE DE AD BE EF  48 69 2C 20 01 02 03 04
48 69 2C 20 20 2C 69 48  20 2C 69 48 20 2C 69 48
```

## Byte Expressions

Byte expressions consist of any number of the following items separated by spaces to delimit literals:

- Numeric literals.
  Numbers in `htor` may be padded or truncated to a certain total size in bytes via the `[size]number` or `number[size]` notation.
  `[size]` on the left indicates the right side of the number's byte-representation should be padded or truncated, and `[size]` on the right means the opposite.
  - Binary literal: `0b...` with size divisible by 8.
  - Decimal literal: `0d...` with arbitrary size.
    Total size is rounded up to the byte.
  - Hexadecimal literal: `0x...`, case insensitive, with size divisible by 2.
    The `0x` may be omitted for convenience.
- String literals.
  Strings in `htor` do not yet support escape sequences, but this is coming soon!
  String are delimited by double quotes and may contain any ascii characters.
- Flip, `<`, and unflip, `>`.
  The operators indicate whether the byte order for the items following them should be flipped.

## Expansions

Expansions, which you can think of as functions, are defined using `@define name arg1 ...`.
Expansions may take zero or more arguments, and expand to the contents of subsequent indented block.

- A macro with zero arguments may be expanded with either `$name` or `$name()`.
- A macro with one or more arguments must be expanded with `$name(arg1, ...)`, where each argument can be any byte expression.
  All arguments are immediately expanded to avoid recursion.

## Repeat

The `@repeat n` macro simply yields the subsequent indented block `n` times.

## Assembly

The `@assembly` macro is an experimental feature that faciliates embedding compiled assembly directly into the payload.
Typical usage would look something like this:

```
@assembly
  mov $0x00000000deadbeef,%rdi
  ret
```

The contents of the assembly block are directly piped into GCC, and the contents of the `.text` field are read and returned.
Currently, this macro is only supported on Linux due to limitations in the ELF library I'm using.
