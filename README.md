# 📝 Compiler Project - Lexer Implementation (Rust)

This repository contains our **lexer** implemented in Rust for the Compiler Construction course assignment.  
We provide **two versions of the lexer**:

- **Handwritten Lexer (`hand`)** → Uses raw string comparisons and a state machine approach (no regex, no third-party libraries).  
- **Regex Lexer (`regex`)** → Uses Rust’s [`regex`](https://crates.io/crates/regex) crate for pattern matching.

---

## 🚀 How to Build & Run

### 1. Clone the repository
```bash
git clone <repo-link>
cd compiler
```
2. Build the project
```bash
cargo build
```
3. Run with Handwritten Lexer
```bash
cargo run -- hand sample.src
```
4. Run with Regex Lexer
```bash
cargo run -- regex sample.src
```
