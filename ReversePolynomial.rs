use std::io;

fn main() {
  //reading the polynomial
  let mut polynomial = String::new();
  
  io::stdin().read_line(&mut polynomial)
      .ok()
      .expect("Failed to read!");
  
  println!("{}", polynomial);
  
  //checking the first character0.
  let polyAppend = concat!("+", polynomial);
  println!("{}", polyAppend);*-

  //splitting it at +
  let list = polynomial.split("+");
  for i in list {
      println!("{}", i); 
  }
  let ch = list.chars().nth(0).unwrap();
  println!("{}", ch);
}
