fn main() {
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("Enter polynomial: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    let result = str::replace(&s, "-", "+-");
    let joined = format!("{}{}","+", &result);
    let l = s.split("");
    let fs = "";
    for i in l {
        if i == "+" {
            format!("{}{}", "+", &fs);
        }
        println!("{}{}", i, fs);
    }
    println!("{}", joined);
}