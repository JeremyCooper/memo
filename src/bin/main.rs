extern crate memo;

use std::io::{stdin, SeekFrom, Seek};
use std::fs::OpenOptions;
use memo::MemoBox;

fn main() {
    //Determine fibonacci position to calculate
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

    let filename = "memoization.data";

    //Create memoization context
    let mut callbox = MemoBox::new(find_fib);

    //Deserialize
    match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename) {
        Ok(mut file) => {
            match file.seek(SeekFrom::Start(0)) {
                Ok(_) => (),
                Err(e) => {
                    println!("Error seeking file. {}", e);
                },
            }
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
    match OpenOptions::new()
            .write(true)
            .open(filename) {
        Ok(file) => {
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
