// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
use rustc_hash::FxHashMap;
use std::boxed::Box;
// use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

// wrap vector in Box to miminize memory representation of School
pub struct School(FxHashMap<u32, Box<Vec<String>>>);

impl Deref for School {
    type Target = FxHashMap<u32, Box<Vec<String>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for School {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl School {
    pub fn new() -> School {
        School(FxHashMap::default())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let _ = self
            .entry(grade)
            .and_modify(|v| v.push(student.to_owned()))
            .or_insert(Box::new(vec![String::from(student)]));
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut res = self.keys().copied().collect::<Vec<u32>>();
        res.sort();
        res
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        if let Some(v) = self.get(&grade) {
            let mut students = v.clone();
            students.sort();
            *students
        } else {
            vec![]
        }
    }
}
