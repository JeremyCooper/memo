extern crate memo;

use std::io::{Read, Write, stdin};
use std::fs::File;
use memo::MemoBox;

//TODO: error handling
fn main() {
    println!("Please enter nth fibonacci position to print");
    let mut n = String::new();
    stdin().read_line(&mut n).expect("read_line failed");
    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_)  => {
            println!("You did not enter a number, so you get the 7th position");
            7
        },
    };

    //Create memoization context
    let mut callbox = MemoBox::new(find_fib);

    let filename = "memoization.data";

    //Deserialize
    match File::open(filename) {
        Ok(mut file) => {
            callbox.des(file);
        },
        Err(e) => {
            println!("Error opening file {}: {}", filename, e);
        },
    }

    //Call memoization context twice
    let result = callbox.call(n);
    println!("The {}th number is: {}", n, result);
    println!("Attempting resolve...");
    let result = callbox.call(n);
    println!("The {}th number is: {}", n, result);

    //Serialize
    match File::open(filename) {
        Ok(mut file) => {
            callbox.ser(file);
        },
        Err(e) => {
            println!("Error opening file {}: {}", filename, e);
        },
    }

    //Dump debuging information to stdout
    callbox.dump_table();
}

fn find_fib(n: u32) -> u32 {
    match n {
        0 | 1 => n,
        _     => {
            find_fib(n-1)+find_fib(n-2)
        },
    }
}
