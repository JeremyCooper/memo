/* 
 * Simple memoization library
 */

//! Call function through memoization wrapper
//! Memoization wrapper will search data structure
//! for previous cached calls and return stored
//! value if found.

extern crate serde;
extern crate bincode;

//TODO: Add serialization and deserialization, serde

use std::hash::Hash;
use std::collections::HashMap;
use std::fmt::Display;
use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use bincode::{serialize_into, deserialize_from, Infinite};
use std::io::{Read, Write};

pub struct MemoBox<I, O, F> {
    data: HashMap<I, O>,
    function: F,
}
impl <I, O, F> MemoBox<I, O, F> where
    I: Hash + Eq + Clone + Display + Serialize + DeserializeOwned,
    O: Clone + Display + Serialize + DeserializeOwned,
    F: Fn(I) -> O {
    pub fn new(f: F) -> Self {
        MemoBox {
            data: HashMap::new(),
            function: f,
        }
    }
    pub fn call(&mut self, input: I) -> O {
        let ref fun = self.function;
        let output =
            self.data.entry(input.clone())
            .or_insert_with(|| (fun)(input));
        output.clone()
    }
    //TODO: ser/des take Read/Write satisfying arguments passed from main
    pub fn des<R: Read>(&mut self, mut handle: R) {
        match deserialize_from(&mut handle, Infinite) {
            Ok(values) => self.data = HashMap::from(values),
            Err(e) => {
                println!("Encountered error while deserializing. {}", e);
            },
        }
    }
    pub fn ser<W: Write>(&self, mut handle: W) {
        if let Err(e) = serialize_into(&mut handle, &self.data, Infinite) {
            println!("Encountered error while serializing. {}", e);
        }
    }
    pub fn dump_table(&self) {
        for (key, value) in self.data.iter() {
            println!("I: {}, O: {}", key, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        fn square(x: i32) -> i32 { x*x }
        let mut callbox = CallBox::new(square);
        let result = callbox.call(5);
        assert_eq!(result, 25);
        callbox.dump_table();
    }
}
