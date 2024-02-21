
fn main() {
  for i in 1..10 {
    let odd_or_even =
      if i % 2 == 0 {
        "even"
      } else {
        "odd"
      };

    // let mut odd_or_even = "";

    // if i % 2 == 0 {
    //   odd_or_even = "even";
    // } else {
    //   odd_or_even = "odd";
    // }
    println!("counted {} is {}", i, odd_or_even);
  }

}