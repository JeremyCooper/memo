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
use std::fs::File;

pub struct MemoBox<I, O> {
    data: HashMap<I, O>,
    function: fn(I) -> O,
}
impl <I: Hash + Eq + Clone + Display, O: Clone + Display> MemoBox<I, O> where
    I: Serialize + DeserializeOwned,
    O: Serialize + DeserializeOwned {
    pub fn new(callable: fn(I) -> O) -> Self {
        MemoBox {
            data: HashMap::new(),
            function: callable,
        }
    }
    pub fn call(&mut self, input: I) -> O {
        let fun = self.function;
        let output =
            self.data.entry(input.clone())
            .or_insert_with(|| (fun)(input));
        output.clone()
    }
    pub fn des(&mut self) {
        let mut file = File::open("memoization.data").unwrap();
        self.data = HashMap::from(
            deserialize_from(&mut file, Infinite).unwrap()
        );
    }
    pub fn ser(&self) {
        let mut buffer = File::create("memoization.data").unwrap();
        serialize_into(&mut buffer, &self.data, Infinite).unwrap();
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
