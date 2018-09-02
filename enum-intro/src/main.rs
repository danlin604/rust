enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

#[derive(Debug)]
enum Message {
  //Quit,
  //Move { x: i32, y: i32 },
  Write(String),
  //ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self) {
    // method body would be defined here
    println!("{:#?}", self);
  }
}

// Already Included
// enum Option<T> {
//   Some(T),
//   None
// }

fn main() {
  let _home = IpAddr::V4(127, 0, 0, 1);
  let _loopback = IpAddr::V6(String::from("::1"));

  let m = Message::Write(String::from("hello"));
  m.call();

  let some_number = Some(5);
  let some_string = Some("a string");

  let absent_number: Option<i32> = None;
}
