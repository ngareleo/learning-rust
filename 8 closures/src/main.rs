fn main() {
    let first_closure = || println!("I'm a closure!");
    first_closure();
    closures_with_ownership();
    closures_with_move();
}



fn closures_with_ownership() {
    let mut arr = vec![1, 2, 3];

    let mut name = String::from("Leo");

    let mut closure = || {
        name.push_str("nardo"); // because of the mutation here, the cloure needs a mutable reference to name
        arr.push(4); // same here for the array
        println!("arr: {:?}", arr);
    };

    closure();

    println!("array is {:?}", arr);
    println!("name is {}", name);
}

fn closures_with_move() {
    let mut arr = vec![1, 2, 3];
    let name = String::from("Leonardo");

    let mut closure = move || {
        arr.push(4);
        println!("name is {}", name); // only needs a immutable reference to name
        println!("arr: {:?}", arr);
    };

    closure();
    // println!("name is {}", name); // this here will cause the compiler to panic
    // println!("Array is {:?}", arr); // this here will cause the compiler to panic
    // the move key word will move all the references into the closure
}


