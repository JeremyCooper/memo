/* 
 * Simple memoization library
 */

//! Call function through memoization wrapper
//! Memoization wrapper will search data structure
//! for previous cached calls and return stored
//! value if found.

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

//TODO: Add serialization and deserialization, serde

use std::hash::Hash;
use std::collections::HashMap;
use std::fmt::Display;
use serde::ser::Serialize;
use serde::de::Deserialize;
use bincode::{serialize, deserialize, Infinite};

//impl<K, V> Serialize for HashMap<K, V>
//    where K: Serialize,
//          V: Serialize
//{
//    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//        where S: Serializer
//    {
//        let mut map = serializer.serialize_map(Some(self.len()))?;
//        for (k, v) in self {
//            map.serialize_entry(k, v)?;
//        }
//        map.end()
//    }
//}
//pub struct MemoBox<I: Hash + Eq + Clone + Display, O: Clone + Display> {
pub struct MemoBox<I, O> {
    data: HashMap<I, O>,
    function: fn(I) -> O,
}
impl <'a, I: Hash + Eq + Clone + Display, O: Clone + Display> MemoBox<I, O> where
    I: Serialize + Deserialize<'a>,
    O: Serialize + Deserialize<'a> {
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
    pub fn des(&self, encoded: Vec<u8>) {
        let decoded: HashMap<I, O> = deserialize(&encoded).unwrap();
    }
    pub fn ser(&self) {
        let encoded: Vec<u8> = serialize(&self.data, Infinite).unwrap();
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
