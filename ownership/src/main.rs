fn main() {
    println!("Hello, world!");
    let mut x = "mush";
    let y = x; // Here, `y` receives a copy of `x`'s value ("mush"). Changing `x` afterward doesn’t affect `y` since `y` has its own independent copy. Unlike with `String`, explicit cloning isn’t needed because this type has a fixed size.
    x = "Don";
    println!("X is {x} and Y is {y}");

    // In Rust, a `String` has three main parts: a pointer to the memory storing its contents, its length, and its capacity (the allocated memory size in bytes).
    let s1: String = String::from("hello");
    // let s2 = s1; // This copies `s1`'s pointer, length, and capacity to `s2`, without duplicating the actual data in heap memory to avoid the cost of reallocation.
    // Here, `s1` is "moved" to avoid a double-free error. Since both `s1` and `s2` would point to the same memory, Rust deactivates `s1`, making `s2` the sole owner. Accessing `s1` afterward results in an error due to this transfer of ownership.
    println!("s1 is {s1}");
  
    // For a deep copy of heap data, cloning must be done explicitly.
    let s3 = s1.clone(); // Cloning creates a new allocation on the heap, which is computationally expensive.
    println!("s3 is {s3}");
    // take_owner(s1);
    // println!("x is {s1}"); // This will cause an error since `s1`’s ownership has moved to `take_owner()`, making `s1` inaccessible.

    let d: i32 = 1;
    let result = sum(d);
    println!("sum is {result}");
    println!("d is {d}"); // `d` is still accessible since it’s stored on the stack, not the heap. Its value is copied when passed to the `sum` function.
    let o = gives_owner(); // Here, the function transfers ownership of a new `String` to `o`.
    
    println!("returned {o}");

    let mut s1 = takes_and_gives_back_owner(s1);
    println!("s1 (shadowing previous s1 on Line-9) has regained ownership after being passed to the `takes_and_gives_back_owner` function: {s1}");
    let (d, r) = use_tuple_owner(d);
    print!("d is {d}, r is {r}");
    
    // REFERENCES AND BORROWING
    let len_of_string = borrowed_references(&s1);
    println!("Length of s1: {len_of_string}");

    mutable_borrowed_references(&mut s1);
    // Only one mutable reference is allowed at a time.
    // let df = &mut s1;
    // let d3 = &mut s1;
    println!("Modified version of s1: {s1}");

    // Mutable references can be created in different scopes without conflict.
    {
        let d3 = &mut s1;
    }
    
    let d4 = &mut s1;

    let d6 = &s1;
    print!("d6 {d6}");
    let d7 = &mut s1;
    // print!("d6 {d6}"); // This will cause an error as `d6` is an immutable reference of `s1`, but `d7` makes it a mutable reference.

}

fn take_owner(s: String) {
    println!("Owner is {s}");
}

fn gives_owner() -> String {
    let t = String::from("Gives Owner");
    t
}

fn takes_and_gives_back_owner(s: String) -> String {
    println!("Owner of s is {s} here");
    s
}

fn sum(s: i32) -> i32 {
    s + 10
}

fn use_tuple_owner(s: i32) -> (i32, i32) {
    let d: i32 = s + 10;
    (s, d)
}

fn borrowed_references(s: &String) -> usize {
    s.len()
}

fn mutable_borrowed_references(s: &mut String) {
    s.push_str(" newly added string");
}

// This function will cause an error because it tries to return a reference to a variable that will be deallocated once the function ends.
// fn dangle() -> &String {
//     let s = String::from("Hello world");
//     &s
// }
