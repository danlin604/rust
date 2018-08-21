fn main() {

  // create instance
  let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  };

  user1.email = String::from("anotheremail@example.com");

  println!("{}", user1.email);

  // create instance through fn
  let user2 = build_user(String::from("user2@example.com"), String::from("user2"));
  println!("{}", user2.email);


  // create instance from other instances
  let user3 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
  };

  // less code
  let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
  };

  println!("email: {}, active: {}", user2.email, user2.active);

  // tuple struct
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);

  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
}

struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}