use std::{thread, time::Duration, sync::{mpsc, Mutex, Arc}};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("[.] Spawn thread: count {i}.");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap(); placing it here will execute the child thread then the main

    for i in 1..7 {
        println!("[?] Main thread, count {i}.");
        thread::sleep(Duration::from_millis(1));

    }

    handle.join().unwrap();

    

    // let handle_with_move = thread::spawn(move || {
    //     names.iter().for_each(move |name| {
    //         println!("{name}");
    //     });
    // });


    // handle_with_move.join().unwrap();

    let (tx, rx) = mpsc::channel();
    let cats_tx = tx.clone();

    let handle_with_channel = thread::spawn(move || {
        let names = vec![String::from("John"), String::from("Jane"), String::from("Liz")];
        for name in names {
            tx.send(format!("[*] From humans thread. {name}.")).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    let handle_catnames_with_channel = thread::spawn(move || {
        let catnames = vec![String::from("Calico"), String::from("Frankie")];
        for name in catnames {
            cats_tx.send(format!("[:] From cats thread. {name}.")).unwrap();
            thread::sleep(Duration::from_secs(5));
        }
    });


    for message in rx {
        println!("{message}");
    }

    handle_with_channel.join().unwrap();
    handle_catnames_with_channel.join().unwrap();


    // mutexes
    let counter = Arc::new(Mutex::new(0));
    // Arc creates an atomic Cell
    // Mutexes allow threads to share data by acquiring locks

    let mut handles = vec![];
    for _ in 0..13 {
        let counter = Arc::clone(&counter); // each tread must make an atomic copy of the mutex
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); //acquire the lock here
            *num +=1 ; // lock is given up here
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("[+] Result, {}", *counter.lock().unwrap());

}
