
const LIMIT: u32= 23;

fn main() {
    let a: usize = 32; // depends on architecture 64-bit or 32-bit
    let b: u32 = 3_434; // use _ for separation
    let c = 0x323; //hex
    let d = 0o232; // octal
    let e = 0b10010; // binary
    println!("{a}");
    println!("{b}");
    println!("{c}");
    println!("{d}");
    println!("{e}");

    let tup: (i8, f32) = (12, 12.3); // tuple. Immutable array
    let (f, g) = tup; // destructuring
    println!("{f}");
    println!("{g}");

    let ls = [3; 5]; // five duplicates of 3

    // error
    // let x = 6;
    // println!("{x}");
    // x = 7;
    // println!("{x}");

    let mut x = 6;
    println!("{x}");
    x = 7;
    println!("{x}");

    // constant

    println!("This is a constant {LIMIT}");

    // shadowing
    // Rust allows you to re-declare a variable with the same name
    // The variable value is changed but uses the same memory location
    // Also allows us to change the type


    let y = 10;
    {
        let y = 13;
        println!("Value of Y is {y} in local scope")
    }
    println!("Value of Y is {y} in global scope");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces {spaces}");

    // let mut spaces = "   ";
    // spaces = spaces.len(); // type violation

}
