extern crate memo;

use std::io;
use memo::MemoBox;

fn main() {
    println!("Please enter nth fibonacci position to print");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("read_line failed");
    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_)  => {
            println!("You did not enter a number, so you get the 7th position");
            7
        },
    };

    let mut callbox = MemoBox::new(find_fib);

    callbox.des();

    let result = callbox.call(n);
    println!("The {}th number is: {}", n, result);
    println!("Attempting resolve...");

    let result = callbox.call(n);
    println!("The {}th number is: {}", n, result);

    callbox.ser();
}

fn find_fib(n: u32) -> u32 {
    match n {
        0 | 1 => n,
        _     => {
            find_fib(n-1)+find_fib(n-2)
        },
    }
}
