fn main() {
  let s1 = String::from("hello");
  let len = calculate_length(&s1); // passing reference
  println!("The length of '{}' is {}.", s1, len);

  let mut s = String::from("hello");
  change(&mut s);
  println!("{}", s);

  // you can only have one mutable reference to a particular piece of data in a particular scope (prevents data races)

  // you can have multiple mutable references, but not simultaneous ones

  let mut s = String::from("hello");
  {
    let r1 = &mut s;
  } // r1 goes out of scope
  let r2 = &mut s;

  let mut s = String::from("hello");
  let r1 = &s; // no problem
  let r2 = &s; // no problem
  //let r3 = &mut s; // BIG PROBLEM

  let s = dangle();
  println!("{}", s);
}

fn calculate_length(s: &String) -> usize {
  s.len()
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}

fn dangle() -> String {
  let s = String::from("hello");
  s
}