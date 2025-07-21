## Programming Languages Intermediate Representation

[![Status](https://github.com/vaivaswatha/pliron/actions/workflows/ci.yml/badge.svg)](https://github.com/vaivaswatha/pliron/actions/workflows/ci.yml)

`pliron` is an extensible compiler IR framework, inspired by [MLIR](https://mlir.llvm.org/docs/LangRef/)
and written in safe Rust.

### Build and Test
* Install the [rust toolchain](https://www.rust-lang.org/tools/install).
* `cargo build` and `cargo test` should build the compiler and run the testsuite.
* To see a simple IR constructed (by the [print_simple](tests/ir_construct.rs) test),
  use the following command:

      cargo test print_simple -- --show-output

  It should print something like:
  ```mlir
  builtin.module @bar 
  {
    ^block_1v1():
      builtin.func @foo: builtin.function <()->(builtin.integer si64)> 
      {
        ^entry_block_2v1():
          c0_op_3v1_res0 = test.constant builtin.integer <0: si64>;
          test.return c0_op_3v1_res0
      }
  }
  ```
* `pliron` provides an [LLVM Dialect](pliron-llvm/README.md) and
consequently an [`llvm-opt` tool](pliron-llvm/llvm-opt/README.md)
that can parse LLVM-IR bitcode into the LLVM dialect and output
LLVM-IR bitcode.

### Using the Library
`pliron`, is under active development. All effort is made to ensure that the code is well tested
and of production quality. Current efforts are directed at completing the LLVM dialect, so that
at least one backend exists for `pliron` and it becomes usable. If someone wants to use just the compiler
infrastructure, without requiring an LLVM backend, just add a dependence to the [crate](https://crates.io/crates/pliron)
in your Rust project.

### Documentation
* Introduction and motivation are covered in the [introductory wiki article](https://github.com/vaivaswatha/pliron/wiki/Introduction).
* The wiki also has a [comparison](https://github.com/vaivaswatha/pliron/wiki/Comparison-with-other-compiler-frameworks) of `pliron`
with other compiler projects, touching upon some design decisions.
* Code documentation can be found on
  [docs.rs](https://docs.rs/pliron/latest/pliron/).

### Some talks on `pliron`
* [pliron: An Extensible IR Framework in Rust - IICT'24](https://www.youtube.com/watch?v=LobYuwcUaZA)
* [Rust(ing) the Future of Compilers: Pliron as the MLIR Alternative (No C/C++)](https://www.youtube.com/watch?v=rRgYGBAhKQ0)
* [Pliron Rust Workshop (6 sessions)](https://www.youtube.com/watch?v=6EjMWJ2PY-o)

![pliron-logo](https://github.com/user-attachments/assets/adfaaeed-775f-4290-92fd-93d7c9b4fd12)
