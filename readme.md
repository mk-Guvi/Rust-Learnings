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

- **Semantic Versions**:
   Rust crates use semantic versioning. For example, in version 0.8.6, the 0 refers to the major version, indicating changes that might remove old functionalities. The 8 refers to the minor version, representing updates that add new features without breaking existing functionality. The 6 refers to the patch version, which addresses bug fixes.  


- **Shadowing Variables**:  
   In Rust, you can declare a new variable with the same name as a previous one, with the second declaration "shadowing" the first. This means the compiler only recognizes the most recent version of the variable. Shadowing allows you to reference the original value in the new variable. A common use case is making a variable immutable after performing some operations. Unlike with `mut`, shadowing allows you to change the type of the value. This behavior is demonstrated in the guessing game program.  
   
   Example: `let a = 2; let a = a + 2; println!("a value is {a}");` // Outputs 4.  
   
   Although you could achieve a similar result by using different variable names (e.g., `let a = 2; let a_readonly = a;`), this can create confusion over which variable to use. Shadowing provides a cleaner and more efficient solution.


### Scalar Types in Rust

   In Rust, a **scalar type** represents a single value. Rust provides four primary scalar types:

   1. **Integer**
   2. **Floating Point Numbers**
   3. **Booleans**
   4. **Characters**

   #### 1. Integer
      An integer is a number without decimal points. There are two kinds of integers in Rust:

      - **Signed integers (`i32`)**: These store both positive and negative values. Rust uses 2's complement representation, meaning if the first bit is 0, the number is negative; otherwise, it's positive. The range for signed integers is `-(2^(n-1))` to `2^(n-1) - 1`, where `n` is the number of bits (e.g., for `i8`, the range is `-128` to `127`).
      
      - **Unsigned integers (`u32`)**: These store only positive values, from `0` to `2^n - 1` (e.g., `u8` ranges from `0` to `255`).

      Types of integers:
      - `i8`, `i16`, `i32`, `i64`, `i128`, `isize` (signed integers)
      - `u8`, `u16`, `u32`, `u64`, `u128`, `usize` (unsigned integers)

      For example, `i8` can hold values from `-128` to `127`. You can specify integer types using a suffix, such as `28u8`. Rust allows the use of underscores as visual separators for readability, like `1_000_000`.

      ##### Integer Literals:
      - **Decimal**: `10_000`
      - **Hexadecimal**: `0xff`
      - **Octal**: `0o77`
      - **Binary**: `0b1111_0000`
      - **Byte (`u8` only)**: `b'A'`

      #### Handling Integer Overflow
      In Rust, integer overflow errors are raised in Debug mode (`cargo run`), but in Release mode (`cargo run --release`), instead of throwing an error, the value undergoes 2's complement wrapping.

      Rust provides several methods to handle integer overflow, especially in **debug mode**:

      - **`wrapping_add()`**: Wraps around on overflow.
      ```rust
      let b: u8 = random_number().wrapping_add(200);
      println!("b is {b}");
      ```

      - **`overflowing_add()`**: Returns a tuple where the second value is a `bool` indicating whether an overflow occurred.
      ```rust
      let (d, h): (u8, bool) = random_number().overflowing_add(200);
      println!("D is {d}, H is {h}");
      ```

      - **`checked_add()`**: Returns `None` if the addition would overflow.
      ```rust
      let c = match random_number().checked_add(200) {
            Some(num) => num,
            None => {
               println!("Cannot add number");
               return;
            }
      };
      println!("C is {c}");
      ```

      This allows safe and predictable behavior when working with integer operations.
   
   #### 2. **Floating-Point Types**

      Rust supports two floating-point types: `f32` and `f64`, with `f64` being the default. All floating-point numbers in Rust are signed, meaning they can represent both positive and negative values.

      - **`f32`**: Single precision (32 bits)
      - **`f64`**: Double precision (64 bits)

   #### 3. **Numeric Operations**

      Rust provides all the basic arithmetic operations:

      - **Addition** (`+`)
      - **Subtraction** (`-`)
      - **Multiplication** (`*`)
      - **Division** (`/`)
      - **Remainder** (`%`)

      For division, if both operands are integers, the result will be an integer as well, and any decimal part will be truncated. If you need the result to include the decimal portion, you must specify floating-point types. For example:

      ```rust
      let a = 5 / 2;        // Result: 2 (integer division)
      let b: f32 = 5_f32 / 2_f32;  // Result: 2.5 (floating-point division)
      ```

   #### 4. **Boolean Type**

      Rust's boolean type represents a value that can be either `true` or `false`. It is typically used in conditional statements and logic operations.
   
   #### 5. **Character Type**

      In Rust, strings are enclosed in double quotes, whereas characters use single quotes. Rust's `char` type represents a single Unicode scalar value. It is 4 bytes in size and can represent a wide range of characters, including letters, numbers, and various special symbols.

      **Example:**

      ```rust
      let c = 'A';
      let heart = '❤';
      let smiley = '😊';
      ```

### Compound Types in Rust

   Compound types allow grouping multiple values into a single type. Rust provides two primitive compound types: **tuples** and **arrays**.

   #### 1. Tuples

   Tuples can store multiple values of different types, and their length is fixed. A special tuple with no values is called a "unit."

   #### 2. Arrays

   Arrays store multiple values of the same type and are also of fixed length in Rust.

   **Examples:**

   ```rust
   let a = [1, 2, 3];
   let b: [u32; 3] = [2, 3, 4];    // Array of 3 u32 elements
   let c = [10; 5];    // Creates an array with 5 elements, all initialized to 10
   ```

   Accessing an index that is out of bounds will result in a **runtime error**. While the code will compile successfully, an error will be thrown if you attempt to access an invalid index during execution.


### Rust Functions

   - **Functions** 
      In Rust, functions are defined using the `fn` keyword, with names written in snake_case. A function signature specifies its name, parameters, and return type.

      Example:
         ```rust
         fn main() {
            another_function();
         }

         fn another_function() {
            println!("Another function.");
         }
         ```

   - **Parameters** are defined in the function signature, and their types must be specified:
      ```rust
      fn another_function(x: i32) {
         println!("The value of x is: {x}");
      }
      ```

   - **Statements** perform actions but don’t return values, while **expressions** must return a value.The function declaration itself is a statement.
      ```rust
      let y = {
         let x = 3;
         x + 1 // Expression returns 4
      };
      ```

      ```rust
      let y = {
         let x = 3;
         return x + 1; // Expression returns 4
      };
      ```

   - Functions can have **return values**, specified with `->`. The last expression is returned:
      ```rust
      fn five() -> i32 {
         5 // Implicit return
      }
      ```

   - Avoid adding semicolons to the final expression, as it converts it into a statement and prevents returning a value:
      ```rust
      fn plus_one(x: i32) -> i32 {
         x + 1 // Correct
      }
      ```

### Control Flows

   - **If Expressions**

      In Rust, the blocks associated with an `if` statement are called **arms**. The condition in an `if` expression must always evaluate to a boolean. For example:

      - Correct: `let a = 1; if (a == 1) { ... }`
      - Incorrect: `if (a) { ... }` (since `a` is not a boolean)

      In Rust, `if` is an **expression**, meaning it must return a value, unlike a statement. When using `if` as an expression, an `else` block is required, and both the `if` and `else` arms must return the same data type.

      **Example:**

      ```rust
      let y = 1;
      let x = if y > 1 {
         true
      } else {
         false
      };
      ```
   - **Loops**

   Rust provides three types of loops: `loop`, `while`, and `for`.

      - **Loop Expression**

         The `loop` keyword tells Rust to continuously execute a block of code. You can use `break` to exit the loop and `continue` to skip to the next iteration. When using `loop` as an expression, you need to return a value. 

         **Example:**

         ```rust
         let mut n = 0;
         let x = loop {
            n += 1;
            if n > 2 {
               break 2;
            }
         };
         ```

         #### **Labeled Loops**

         You can assign labels to loops, which allow you to control and break out of specific loops in nested loop scenarios. If you want to break the parent loop based on conditions in a child loop, you can use a label.

         **Example:**

         ```rust
         let mut n = 0;
         let x = 'first_loop: loop {
            n += 1;

            if n > 10 {
               break 2;
            }

            loop {
               if n == 5 {
                     break 'first_loop 5;
               }
            }
         };
         ```

         In this example, the inner loop breaks the outer `first_loop` when `n` equals 5.
      
      - **While Loop**

         Use a `while` loop when you know the exit condition but not the exact number of iterations.

         #### Example: Loop vs While

         **Using `loop`**:
         ```rust
         let mut count = 0;
         loop {
            count += 1;
            if count == 5 {
               break;
            }
         }
         ```

         **Using `while`**:
         ```rust
         let mut count = 0;
         while count < 5 {
            count += 1;
         }
         ```

         **When to use:**
         - Use `loop` for indefinite repetition with manual control over exit.
         - Use `while` when the loop depends on a condition to exit automatically.

      - **For Loop Expression**

         you can use a `while` loop to iterate over an array, it requires checking the index on each iteration, which is less efficient. Using a `for` loop is a better approach since it automatically handles array bounds and is more efficient.

         #### Example: Iterating over an array with `while` vs `for`

         **Using `while`:**
         ```rust
         let arr = [1, 2, 3, 4, 5];
         let mut index = 0;
         while index < arr.len() {
            println!("{}", arr[index]);
            index += 1;
         }
         ```

         **Using `for`:**
         ```rust
         let arr = [1, 2, 3, 4, 5];
         for element in arr {
            println!("{}", element);
         }
         ```

         #### Iterating with a Range in `for`

         You can also iterate over a range of numbers using a `for` loop.

         ```rust
         for i in 0..=5 {
            println!("{}", i);
         }
         ```

         In this example, the loop iterates from `0` to `5`. 

# Rust Memory Management: Ownership, References, and Borrowing

   Ownership is the set of rules that defines how the rust manages the memory. Memory is manages through a system of ownership with a set of rules that compiler checks. If any of the rules is violated, the program won't compile.

   #### **Stack vs Heap(Allocation)**

      The **stack** is used to store data of fixed size, such as `let a = 10;`. In contrast, the **heap** is used for data with a variable size, like `let b = String::new()`, where the size can change based on user input.

      **Example:**

      ```rust
      let a = 10;
      let b = String::new();
      ```

      In this example:
      - `a` (with a value of 10) is stored directly on the stack.
      - `b` is stored on the stack as a reference, while its actual data is allocated in the heap. This allows for dynamic memory management based on the input size.

      ```rust
      let mut s = "Rust";
      ```
      Here, `s` is stored on the **stack**, and while you can change its value entirely, you cannot modify or adjust its contents (like pushing or popping).

      ```rust
      let mut d = String::from("Rust");
      d.push_str(" is a language!");
      ```
      In this case, `d` is stored on the **heap**, allowing you to dynamically modify its size, like pushing or popping characters.    

   ### Key Differences: Stack vs Heap

      | Feature          | Stack                          | Heap                                |
      |------------------|--------------------------------|-------------------------------------|
      | **Memory Size**  | Limited and fixed              | Larger and flexible                 |
      | **Allocation**   | Automatic (managed by the system) | Manual (managed by the programmer)  |
      | **Speed**        | Faster                         | Slower                             |
      | **Scope**        | Local to function/block        | Accessible until explicitly freed   |
      | **Lifetime**     | Short (limited to function/block) | Can persist longer if not deallocated |

   ### Ownership Rules

   - Every value in Rust has a single owner.
   - There can only be one owner at a time.
   - When the owner goes out of scope, the value is dropped.


   In Rust, memory is automatically freed when ownership is removed.

   #### Example:
   ```rust
   fn main() {
      fn call() {
         let a = "Rust";//These are immutable meaning you can not push or append the characters
      }
      call();
   }
   ```

   In this example, the memory for `a` is automatically freed once the `call` function completes and the variable goes out of scope. You can manually free memory using `drop(a)`, but Rust automatically calls this function when ownership is removed, such as when the owner goes out of scope.


   ### 1. Ownership

   In Rust, each piece of data has a single owner: a variable that manages its memory. When ownership of data is transferred (or "moved"), the original variable can no longer access it.

   #### Example

   ```rust
   fn main() {
      let s1 = String::from("hello");
      let s2 = s1; // Ownership of "hello" is moved from s1 to s2
      
      // println!("{}", s1); // This will cause an error because s1 no longer owns the data
      println!("s2 is: {}", s2); // s2 now has full ownership of the data
   }
   ```

   In this example, `s1` transfers ownership of the `String` "hello" to `s2`. Attempting to use `s1` after the transfer will cause an error, as Rust deactivates `s1` to prevent multiple ownerships, avoiding potential memory issues.

   ### 2. References and Borrowing

      Rust allows data access through references, enabling borrowing data without taking ownership. References can be immutable or mutable, but only one mutable reference to a variable is allowed at a time to prevent data races .A data race is similar to a race condition and happens when these three behaviors occur:

      - Two or more pointers access the same data at the same time.
      - At least one of the pointers is being used to write to the data.
      - There’s no mechanism being used to synchronize access to the data.


   #### Example of Immutable Reference

   ```rust
   fn main() {
      let s = String::from("hello");
      let len = calculate_length(&s); // Passing an immutable reference to s
      println!("The length of '{}' is {}.", s, len);
   }

   fn calculate_length(s: &String) -> usize {
      s.len() // Accessing length of s without taking ownership
   }
   ```

   Here, `calculate_length` borrows `s` without taking ownership. Since the reference is immutable (`&s`), the function can read `s` but not modify it. This allows safe sharing of data.

   #### Example of Mutable Reference

   ```rust
   fn main() {
      let mut s = String::from("hello");
      change(&mut s); // Passing a mutable reference to s
      println!("Modified string is '{}'", s);
   }

   fn change(s: &mut String) {
      s.push_str(", world"); // Mutating the borrowed data
   }
   ```

   In this example, `change` takes a mutable reference to `s` (`&mut s`), allowing it to modify `s`’s contents. Rust enforces that there can only be one active mutable reference to `s` to avoid concurrent mutations, ensuring data integrity.

   ### 3. Ownership and Functions

   When data is passed to a function, ownership is transferred, meaning the function now controls the variable. To regain control, the function must return ownership back.

   #### Example

   ```rust
   fn main() {
      let s = String::from("hello");
      let s = takes_and_gives_back(s); // Ownership is moved and then returned
      println!("s is back with content: {}", s);
   }

   fn takes_and_gives_back(s: String) -> String {
      println!("s is used here: {}", s);
      s // Ownership is returned to the caller
   }
   ```

   The function `takes_and_gives_back` accepts ownership of `s` and then returns it, allowing `s` to be used again in `main`.

   ### 4. Dangling References

   A dangling reference occurs when a reference points to data that has been deallocated. Rust prevents dangling references at compile time by ensuring that data outlives all references to it.

   #### Example

   ```rust
   fn main() {
      let reference_to_nothing = dangle();
   }

   // This function will cause a compile error
   fn dangle() -> &String {
      let s = String::from("hello");
      &s // Error: `s` is dropped here and we return a reference to it
   }
   ```

   Here, `dangle` tries to return a reference to `s`, but `s` is dropped when `dangle` finishes. This would leave the returned reference pointing to invalid memory. Rust’s compiler catches this error, enforcing memory safety by disallowing the creation of dangling references.

   ## Slices :

   # Rust Slices and String Slicing

### Overview
Slices in Rust allow you to reference a part of a collection (like a string or an array) without taking ownership. This is useful for borrowing only part of a collection while still ensuring safety and efficient memory usage.

### Example: Finding the First Word with Slices

In the following example, we have two functions that find the index of the first space in a string to identify the first word. Using slices, we can work with parts of the string directly and avoid issues related to mutable references.

```rust
fn main() {
    let s = String::from("Hello World");
    let result = find_first_word_index(&s); // Finds the first word index.
    println!("Index is {result}");

    // Using slices for specific sections of the string
    let hello = &s[..5];  // "Hello"
    let world = &s[6..];  // "World"
    println!("{hello} {world}");

    // Another example using a function that returns a string slice
    let s2 = String::from("Hello Slice World!");
    let res = find_first_word_index_with_slice(&s2);
    println!("res is {res}");
}
```

### Explanation of Functions

1. **`find_first_word_index`**: This function returns the index of the first space as a `usize`. However, if the original string (`s`) is cleared after the index is found, the result might be invalid.

2. **`find_first_word_index_with_slice`**: By returning a slice (`&str`) instead of an index, this function borrows a part of the original string. Rust ensures that `s` cannot be modified while the slice is in use, preventing issues like modifying the original string while still referencing it.

```rust
fn find_first_word_index(input: &String) -> usize {
    let s = input.as_bytes();
    for (i, &item) in s.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn find_first_word_index_with_slice(input: &str) -> &str {
    let s = input.as_bytes();
    for (i, &item) in s.iter().enumerate() {
        if item == b' ' {
            return &input[..i];
        }
    }
    &input[..]
}
```

### String Literals and Slices

- **String Slice (`&str`)**: A slice of a string, such as `&s[..5]`, represents a subset of a `String` without taking ownership. Useful for borrowing parts of a string while ensuring safety.
- **String Literal (`&str`)**: A string literal, like `"Hello World"`, is a statically allocated slice and is also a `&str`.

### Slices in Rust
A **slice** is a reference to a portion of a collection, like a section of a string or array. Slices let you work with parts of data without taking ownership, ensuring safety and efficiency.

- **Example**: 
  ```rust
  let s = String::from("Hello, World");
  let hello = &s[..5];  // "Hello"
  let world = &s[7..];  // "World"
  ```

### Slices and String Literals in Rust

- **Slices (`&str`)**: A slice is a reference to a part of a collection, like a portion of a `String`. For example, `&s[..5]` creates a slice of the first 5 characters of `s`. Slices let you work with a specific segment of data without taking ownership, ensuring the original data remains unchanged.

- **String Literals (`&str`)**: String literals, like `"Hello World"`, are string slices that are stored directly in the program's binary. They are immutable, statically allocated, and have a fixed size.

Both slices and string literals are `&str` types, allowing efficient, safe borrowing of string data.



      
