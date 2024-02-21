//slice4.rs

fn main() {
  let ints = [1,2,3,4,5,6,7,8,9,10];
  let slice = &ints;

  for s in slice.windows(3) {
    println!("window {:?}", s);
  }

  for s in slice.chunks(5) {
    println!("chunks {:?}", s);
  }
}