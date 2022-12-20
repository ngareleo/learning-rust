
const LIMIT: u32= 23;

fn main() {
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
