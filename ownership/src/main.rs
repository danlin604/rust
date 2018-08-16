fn main() {
  // 1. Each value in Rust has a variable that’s called its owner.
  // 2. There can only be one owner at a time.
  // 3. When the owner goes out of scope, the value will be dropped.

  let mut s = String::from("hello");
  s.push_str(", world!");
  println!("{}", s);

  // memory is automatically returned once the variable goes out of scope; drop is called automtaically at the closing }

  let x = 5;
  let y = x;
  // two variables: x, y. both equals 5

  let s1 = String::from("hello");
  let s2 = s1; // copies ptr, len, capacity, not data on heap
  // a String is made up of three parts: a pointer to memory that holds the content of the string, a length, and a capacity

  // name       value
  // ptr        -> Array [h e l l o]
  // len        5
  // capacity   5

  // println!("{}", s1); // error out; to prevent double release

  // Rust moves the variable when copied: s1 is moved to s2


  // clone (deep copy)
  let s1 = String::from("hello");
  let s2 = s1.clone();
  println!("s1 = {}, s2 = {}", s1, s2);

  // copy
  let x = 5;
  let y = x;
  println!("x = {}, y = {}", x, y);
  // what are copy: all integer types, boolean types, floating types, character types, tuples (with types that are also Copy)

  // ownership and functions
  let s = String::from("hello");
  takes_ownership(s); // s moves into the function
  // s is no longer valid

  let x = 5;
  makes_copy(x);

  // return values and scope
  let s1 = gives_ownership();
  let s2 = String::from("hello");
  let s3 = takes_and_gives_back(s2);

  // return multiple values
  let s1 = String::from("hello");
  let (s2, len) = calculate_length(s1);
  println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
  println!("{}", some_string);
} // memory is freed (String)

fn makes_copy(some_integer: i32) {
  println!("{}", some_integer);
} // Nothing happens

fn gives_ownership() -> String {
  let some_string = String::from("hello");
  some_string
}

fn takes_and_gives_back(a_string: String) -> String {
  a_string
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len();
  (s, length)
}