# Hack Assembler

## Introduction

This project is an implementation of the Hack Assembler, written in Rust. The Hack Assembler translates Hack assembly
language into Hack machine language, which can be executed on the Hack computer described in the "From NAND to Tetris"
course.

## Hack Assembly Language

The Hack assembly language is a simple, low-level programming language designed for the Hack computer. It consists of
two types of instructions:

1. **A-instructions**: Used to set the value of the A register.
    - Syntax: `@value`
    - Example: `@21` sets the A register to 21.

2. **C-instructions**: Used for computations and memory access.
    - Syntax: `dest=comp;jump`
    - Example: `D=M+1;JGT` sets D to M+1 and jumps if greater than zero.

### Hack Assembly Language Contract

- **A-instructions (`@value`)**:
    - `value` can be a non-negative decimal number or a symbolic label.
    - The value sets the address in the A register.

- **C-instructions (`dest=comp;jump`)**:
    - `dest` (optional): Specifies the destination (A, D, M) where the result of the computation is stored.
    - `comp`: Specifies the computation to be performed (
      e.g., `0`, `1`, `-1`, `D`, `A`, `M`, `D+1`, `A-1`, `M-1`, `D+M`, etc.).
    - `jump` (optional): Specifies the jump condition (JGT, JEQ, JGE, JLT, JNE, JLE, JMP).

## Features of the Hack Assembler

- **Symbol Handling**: Supports symbols for variables and labels.
- **Error Checking**: Provides informative error messages for syntax errors and undefined symbols.
- **Optimized Code**: Generates efficient Hack machine code.
- **User-Friendly CLI**: Easy-to-use command-line interface for assembling `.asm` files.

## Installation

To use this assembler, you'll need to have Rust installed on your machine. If you don't have Rust installed, you can get
it [here](https://www.rust-lang.org/tools/install).

Clone the repository and navigate to the project directory:

```sh
git clone https://github.com/veryshyjelly/hack-assembler.git
cd hack-assembler
```

Build the project using Cargo:

```sh
cargo build --release
```

The executable will be located in the `target/release` directory.

## Usage

To assemble a Hack assembly file, run the following command:

```sh
./hack-assembler path/to/yourfile.asm
```

This will generate a binary file with the same name as the input file but with a `.hack` extension, which contains the
Hack machine code.

## Example

Given the following Hack assembly code in `example.asm`:

```asm
// Adds 1 + ... + 100
@i
M=1
@sum
M=0
(LOOP)
@i
D=M
@100
D=D-A
@END
D;JGT
@i
D=M
@sum
M=M+D
@i
M=M+1
@LOOP
0;JMP
(END)
@END
0;JMP
```

Running the assembler:

```sh
./hack-assembler example.asm
```

Will produce the following `example.hack` file:

```plaintext
0000000000010000
1110110000010000
0000000000010001
1110101010001000
0000000000010000
1111110000010000
0000001100010100
1110010011010000
0000000000010010
1110001100001000
0000000000010000
1111110000010000
0000000000010001
1111000010001000
0000000000010000
1111110111001000
0000000000010010
1110101010000111
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you have any improvements or
bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- The creators of the "From NAND to Tetris" course, Noam Nisan and Shimon Schocken, for providing the framework and
  inspiration for this project.
- The Rust community for their excellent documentation and support.
