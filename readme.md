### Rust Commands Overview

1. **Compile a Rust file**:  
   `rustc main.rs`

2. **Run the compiled file on macOS/Linux**:  
   `./main.rs`

3. **Format Rust files**:  
   `rustfmt main.rs`

4. **Create a new Cargo project**:  
   `cargo new hello_cargo`  
   This creates a folder named `hello_cargo` with a basic Cargo setup. All Rust source files should be placed under the `src` directory.

5. **Build the project**:  
   `cargo build`  
   This command creates executable files in the `./target/debug/hello_cargo` directory.

6. **Compile and run the project**:  
   `cargo run`  
   This will compile the code (if necessary) and run the resulting executable from `./target/debug/hello_cargo`.

7. **Check the code for errors**:  
   `cargo check`  
   This command checks the code and reports any errors without producing an executable, making it faster than a full build.

8. **Build an optimized release version**:  
   `cargo build --release`  
   This will create optimized executables in the `./target/release` directory. Since this process takes more time than a standard build, it's typically used for production, while `cargo build` is preferred for development.

### Notes:

- **Macros**:  
  In Rust, using `!` indicates a macro, not a function.  
  Example: `println!("Hello, world!")` is a macro.

- **Standalone Executables**:  
  Rust-generated executable files can run on systems without Rust installed, unlike some other languages (e.g., Python, JavaScript), which require their respective runtime environments.

