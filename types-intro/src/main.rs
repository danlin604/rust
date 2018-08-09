fn main() {
  // numeric
  let guess: u32 = "42".parse().expect("Not a number!");
  println!("Guess: {}", guess);

  // bool
  let t = true;
  let f: bool = false; // with explicit type annotation
  println!("Bool: t: {}, f: {}", t, f);

  // char
  let c = 'z';
  let z = 'ℤ';
  let heart_eyed_cat = '😻';
  println!("Character types: {}, {}, {}", c, z, heart_eyed_cat);

  // tuples
  let tup: (i32, f64, u8) = (1_500, 6.4, 1);
  let (x, y, z) = tup;
  println!("Tuple: x: {}, y: {}, z: {}", x, y, z);
  println!("Tuple: tup.0: {}, tup.1: {}, tup.2: {}", tup.0, tup.1, tup.2);

  //array
  let a = [1, 2, 3, 4, 5];
  println!("Array: a[0]: {} ...", a[0]);
  let a: [i32; 5] = [1, 2, 3, 4, 5];
  println!("Array: a[1]: {} ...", a[1]);
  // panicks when accessing index out of bounds
}
