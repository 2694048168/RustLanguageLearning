## the Rust Programming Language Learning

> Rust learning from scratch, 从零开始学习 Rust 编程语言. 学习 Rust 可以辅助我们对 C++ 有进一步深入的理解和编写更好的 C++ 代码.

**Overview**
- [feature](#features)
- [quick start](#quick-start)
- [simple example](#simple-example)
- [quick debugging](#quick-debugging)
- [useful links](#useful-links)


### **Features**
- [x] Rust install & rustc compiler
- [x] rust-analyzer in VSCode extensions
- [x] Cargo is Rust building and package management
- [x] Rust build/compile/run/debugging/release
- [ ] core concepts of Rust: variables/basic types/functions/comments/control flow


### Quick start
```shell
# 1. Rust downloading and installing
rustc --version
# rustc 1.70.0 (90c541806 2023-05-31)

# Updating and Uninstalling
rustup update
rustup self uninstall

# Local Documentation
rustup doc

# 2. VSCode downloading and installing

# 3. rust-analyzer extension installing

# 4. simple hello_world example
mkdir hello_world && cd hello_world
touch main.rs

# compiler
rustc main.rs
./main

# 5. Rust with Cargo, which is Rust’s build system and package manager.
cargo --version
# cargo 1.70.0 (ec8a8a0ca 2023-04-25)

cargo new hello_cargo
# cargo new hello_cargo --vcs=git
cd hello_cargo
code .

cargo build
cargo run
cargo check

cargo build --release

# Rust source code format
rustfmt --version
# rustfmt 1.5.2-stable (90c54180 2023-05-31)
```

### Simple example
```Rust
// hello_cargo/src/main.rs
fn main(){
    println!("Hello, world.");
    println!("Hello, Rust world.\n");
    println!("Welcome the Rust programming world.\n");
}
```

### Quick debugging
- breakpoint in the 'main.rs' source code file via F9 in VSCode
- Ctrl + Shift + P, and enter >'rust-analyzer:Debug' via VSCode
- and then into the debugging mode!

### Useful links
- [Rust official site](https://www.rust-lang.org/)
- [Rust Download](https://www.rust-lang.org/)
- [The Rust Programming Language online](https://doc.rust-lang.org/book/)
- [Rust By Example online](https://doc.rust-lang.org/rust-by-example/)
- [Rust Language Cheat Sheet](https://cheats.rs/)
- [Rust crates registry](https://crates.io/)
