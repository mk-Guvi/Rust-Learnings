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

9. **Add rand Crate**:
   `cargo add rand`
   This will add rand crate.

10. **Add rand Crate**:
   `cargo doc --open`
   This opens the docs of all the crates used.
### Notes:

- **Macros**:  
  In Rust, using `!` indicates a macro, not a function.  
  Example: `println!("Hello, world!")` is a macro.

- **Standalone Executables**:  
  Rust-generated executable files can run on systems without Rust installed, unlike some other languages (e.g., Python, JavaScript), which require their respective runtime environments.

- **Prelude**:  
  Rust provides a set of commonly used methods and traits that are available in the global scope by default. This collection is known as the Prelude. For example, the `String::new` method is part of the Prelude.

- **Crates**:  
  Crates are collections of Rust code. There are two types of crates: binary crates and library crates. Binary crates can be executed directly, while library crates are meant to be integrated into projects but cannot be executed on their own.

-- **Semantic Versions**:
   Rust crates use semantic versioning. For example, in version 0.8.6, the 0 refers to the major version, indicating changes that might remove old functionalities. The 8 refers to the minor version, representing updates that add new features without breaking existing functionality. The 6 refers to the patch version, which addresses bug fixes.  


-- **Shadowing Variables**:  
   In Rust, you can declare a new variable with the same name as a previous one, with the second declaration "shadowing" the first. This means the compiler only recognizes the most recent version of the variable. Shadowing allows you to reference the original value in the new variable. A common use case is making a variable immutable after performing some operations. Unlike with `mut`, shadowing allows you to change the type of the value. This behavior is demonstrated in the guessing game program.  
   
   Example: `let a = 2; let a = a + 2; println!("a value is {a}");` // Outputs 4.  
   
   Although you could achieve a similar result by using different variable names (e.g., `let a = 2; let a_readonly = a;`), this can create confusion over which variable to use. Shadowing provides a cleaner and more efficient solution.
