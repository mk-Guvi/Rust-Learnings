fn main() {
    println!("Hello, Variables!");
    let mut age = 32;
    println!("Initial Age is {}", age); // Use the initial value.
    age = 321;
    println!("Age is {}", age);

    const PI: i32 = 3; // Constants are always immutable. It's recommended to use uppercase letters with underscores for constants in Rust. Unlike variables declared with `let`, constants must have an explicit type. `let` is scoped, but `const` can be declared globally.
    println!("PI value {PI}");
    // const AGE_MULTIPLER: i32 = 3 * age; // You cannot use non-constant values in the definition of a constant.

    const AGE_BASE: i32 = 20;
    const AGE_MULTIPLIER: i32 = 3 * AGE_BASE;
    println!("Age Multiplied {AGE_MULTIPLIER}"); // Constants are valid throughout the entire program within the scope they are declared in.

    //SHADOWING

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // The value of x in the inner scope is: 12
    // The value of x is: 6
}
