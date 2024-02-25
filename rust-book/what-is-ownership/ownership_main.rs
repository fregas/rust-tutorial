
fn main() {
  let s = String::from("hello");  // s comes into scope

  takes_ownership(s);             // s's value moves into the function...
                                  // ... and so is no longer valid here

  println!("{}", s);

  let x = 5;                      // x comes into scope

  makes_copy(x);                  // x would move into the function,
                                  // but i32 is Copy, so it's okay to still
                                  // use x afterward

  println!("{}", x);                                    

}

fn takes_ownership(some_string: String) {
  println!("{}", some_string);
}

fn makes_copy(some_integer: i32) { // some_integer comes into scope
  println!("{}", some_integer);
} 