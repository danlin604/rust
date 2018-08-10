fn main() {
  println!("Hello, world!");

  another_function(5, 6);

  let x = 5;

  println!("Value of x is: {}", x);

  let y = {
    let x = 3;
    x + 1
  };

  println!("Value of expression y is: {}", y);

  // function return value
  println!("Return value of five is: {}", five());
}

fn another_function(x: i32, y: i32) {
  println!("The value of x is: {}", x);
  println!("The value of y is: {}", y);
}

fn five() -> i32 {
  5
}
