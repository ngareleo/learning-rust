fn main() {

    // String literals
    let string_literal = "This is a string literal"; // immutable
    let string_literal = string_literal.replace("i", "u"); // creates a copy // hence shadowing
    println!("{string_literal}");

    // String in Rust
    let mut string_example = String::from("Leo"); // this exists because string literals are immutables 
    
    string_example.push_str(" Mwenda");
    println!("My name is {string_example}");

    // copying in Rust

    // let name = String::from("Leo");
    // let another_name = name;
    // println!("{name}") // error

    // instead

    let name = String::from("Leo");
    let another_name = name.clone(); // this performs a deep clone of name
    // Rust ensures the programmer explicitly say that "I'm creating a deep clone"
    // Deep copying from heap is very expensive
    println!("Name = {name}. Another name = {another_name}"); 

    {
        let new_name = String::from("Leo Ngare");
        let sample_number = 32;

        print_name(new_name); // because Rust cannot make a deep copy of new_name, its value is moved from main to print_name
        print_number(sample_number); // Rust can afford a deep copy so the value its copied

        // println!("{new_name}"); // error
        println!("{sample_number}");
    }

    {
        // Borrowing
        // In rust, passing a reference in rust is called borrowing. No ownership.
        let new_name = String::from("Ngare");
        print_name_with_reference(&new_name);   // reference the location of new_name without taking ownership
        println!("{new_name}");
    }

    let mut firstname = String::from("Leo");
    change_name(&mut firstname);
    println!("Calling change {firstname}");

    // let s = &mut firstname;
    // let v = &mut firstname; // you cannot borrow again from firstname until the s is exhausted.

    // println!("s {}. v{}", s, v); // cannot borrow more than once

    let s = &mut firstname;
    println!("s {}", s);
    let v = &mut firstname;
    println!("v {}", v);

    // you can create as many immutable references as you want
    // but you cannot create a second mutable reference until you've finished using the first mutable reference

    // slice of a string literal
    let str_literal = "Leo Mwenda"; // stored on the stack
    let string_def = String::from("Leo Mwenda"); // stored on the heap
    let str_literal_slice = &str_literal[..3];
    println!("{str_literal_slice}");
    let str_literal_slice = first_word(&string_def);
    println!("{str_literal_slice}");

    // converting a String to a &str. 

}

fn print_name(name: String) {
    // takes ownership of name
    // name is dropped at the end of the fn call
    println!("Name {name}");
}

fn print_number(number: i32) {
    // takes a copy of number
    println!("Number {number}")
}

fn print_name_with_reference(name: &String){
    // borrows the value. 
    println!("Name {name}");
}

fn change_name(name: &mut String) {
    // will change the value of the borrowed argument
    name.push_str(" Ngare");
}

fn first_word(word: &str) -> &str {
    let bytes = word.as_bytes();
    for (i, &character) in bytes.iter().enumerate() {
        if character == b' ' {
            return &word[..i];
        }
    }
    &word[..]
}