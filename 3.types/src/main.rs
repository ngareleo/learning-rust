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
}
