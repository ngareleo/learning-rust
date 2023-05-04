#![allow(unused)]   

mod models;
use std::{collections::HashMap, time::Instant};

use crate::models::user::User;

fn main() {
    let v: Vec<i32> = Vec::new();
    // same as

    let v = vec![1, 3, 4]; // type annotation is unncesseary
    let mut v = vec![4, 5, 6];
    v.push(54); 

    println!("{:?}", v); // prints inline
    println!("{:#?}", v); // prints vector in a column

    let mut head = &v[0]; // panics if index is out of bounds
    let head = v.get(0);

    match head {
        Some(val) => println!("The head is at, {}", val),
        None => println!("There is no head"),
    }

    for i in &v {
        println!("{i}");
    }

    for i in &mut v { // vector must be mutable
        *i += 50;
    }

    println!("The array: {:?}", v);

    let users = vec![
        User::from(String::from("user1"), String::from("pass1")),
        User::from(String::from("user2"), String::from("pass2")),
    ];
    println!("The users are: {:?}", users);

    let mut scores = HashMap::new();
    scores.insert(String::from("Manchester City"), 71);
    scores.insert(String::from("Arsenal"), 59);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    for (key, value) in &mut scores {
        *value += 10;
    }

    let current_time = Instant::now();

    print!("Score at {:?}: {:?}", current_time, scores); 

    let mut heights = HashMap::new();
    let username = String::from("John");
    let height = 1.8;
    heights.insert(username, height); // the hashmap will take ownership of username and height
    
    let mut weights = HashMap::new();
    let mut jane = String::from("Jane");
    let jane_weight = 85.0;
    weights.insert(&jane, &jane_weight); // the hashmap will take a reference to username2 and height2

    let new_weight = 87.0;
    weights.insert(&jane, &new_weight); // update the value of the key

    // or

    

    jane.push_str("Wendia");

}
