# 📝 Compiler Project - Lexer Implementation (Rust)

This repository contains our **lexer** implemented in Rust for the Compiler Construction course assignment.  
We provide **two versions of the lexer**:

- **Handwritten Lexer (`hand`)** → Uses raw string comparisons and a state machine approach (no regex, no third-party libraries).  
- **Regex Lexer (`regex`)** → Uses Rust’s [`regex`](https://crates.io/crates/regex) crate for pattern matching.

---

## 📂 Repository Structure

src/
├── main.rs # Entry point: runs either handwritten or regex lexer
├── lexer.rs # Handwritten lexer implementation
├── regex_lexer.rs # Regex-based lexer implementation
├── token.rs # Token definitions
sample.src # Example input file
README.md # This file

yaml
Copy code

---

## 🚀 How to Build & Run

### 1. Clone the repository
```bash
git clone <your-repo-url>
cd compiler
2. Build the project
bash
Copy code
cargo build
3. Run with Handwritten Lexer
bash
Copy code
cargo run -- hand sample.src
4. Run with Regex Lexer
bash
Copy code
cargo run -- regex sample.src
