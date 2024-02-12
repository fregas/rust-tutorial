fn factorial(n: u64) -> u64{
  if n == 0 {
    1
  } else {
    n * factorial(n-1)
  }

}

fn main() {
  let fact = factorial(5);
  println!("the factorial is {}", fact);
}