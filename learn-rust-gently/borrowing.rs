
fn main() {
  let mut first = 100;

  let mut second = do_stuff(&first);

  println!("first {}", first);
  println!("second {}", second);

}

fn do_stuff(x: &mut i32) -> &mut i32{
  *x = *x + 1;
  *x
}
  