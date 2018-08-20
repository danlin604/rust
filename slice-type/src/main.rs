fn main() {
  // brittle example
  // let mut s = String::from("hello world");
  // let word = first_word(&s);
  // s.clear(); // word and s is out of sync

  // String slice: reference to part of a string
  let s = String::from("hello world");
  let hello = &s[..=4];
  let world = &s[6..];

  // Using slice
  let my_string = String::from("hello world");
  let word = first_word_slice(&my_string[..]);
  let my_string_literal = "hello world";
  let word = first_word_slice(&my_string_literal[..]);
  let word = first_word_slice(my_string_literal);
  // s.clear(); // error
  println!("{}", word);
}

// fn first_word(s: &String) -> usize {
//   let bytes = s.as_bytes();
//   for (i, &item) in bytes.iter().enumerate() {
//     if item == b' ' {
//       return i;
//     }
//   }
//   s.len()
// }

fn first_word_slice(s: &str) -> &str {
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i];
    }
  }
  &s[..]
}