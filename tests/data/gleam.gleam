//// 34 lines 24 code 4 comments 6 blanks

import gleam/option.{Option, None}
import gleam/io

pub type LoadedBool {
  Yup
  AlsoYup
}

pub external type Person

pub opaque type Cat {
  Cat(
    name: String,
    age: Int,
    is_cute: LoadedBool,
    owner: Some(Person),
  )
}

pub fn main() {
  let jane = // Here is a quote "
    new_kitten(called: "Jane") 
  let kira = new_kitten(called: "Kira")
  io.println("Two kitties!")
}

/// A new baby kitten
///
fn new_kitten(called name: String) -> Cat {
  // No owner yet!
  Cat(name: name, age: 0, is_cute: Yup, owner: None)
}
