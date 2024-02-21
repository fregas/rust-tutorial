// fun4.rs

fn modifies(x: &mut f64) {
  *x = *x * 10.0;
  //*x = 1.0;
  *x = *x + 90.0;
}

fn main() {
  let mut res = 10000.0;
  modifies(&mut res);
  println!("res is {}", res);
}