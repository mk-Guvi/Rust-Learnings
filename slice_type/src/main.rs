fn main() {
    println!("Hello, world!");
    let mut s=String::from("Hello World");
    let result=find_first_word_index(&s);
    //s.clear();//although s is cleared the length returned by result is not changed although s reference is passed in the fn  which an issue. these happens because of scope of s in the function ends once the function is executed.This can be solved by using slices
    println!("Index is {result}");

    //A string slice is a part of string 
    let hello=&s[..5]; //start to 5
    let world=&s[6..]; //6 to end
    //s.clear();//this will now throw error as the reference of s is still valid and clear requires mutable reference 
    println!("{hello} {world}");

    let s2=String::from("Hello Slice World!");
    let res=find_first_word_index_with_slice(&s2);
    println!("res is {res}");
}

fn find_first_word_index(input:&String) -> usize{
     let s=input.as_bytes();//this return each character
     for (i,&item) in s.iter().enumerate(){
        if item == b' '{
            return i;
        }   
     }
     s.len()
}


fn find_first_word_index_with_slice(input: &str) -> &str{
    let s=input.as_bytes();//this return each character
    for (i,&item) in s.iter().enumerate(){
       if item == b' '{
           return &input[..i];
       }   
    }
    &input[..]
}