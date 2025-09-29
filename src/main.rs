use std::io;


fn main() {
let mut num1 = String::new();
let mut num2 = String::new();

println!("Enter num1: ");
io::stdin().read_line(&mut num1).expect("Fail to read information");

println!("Enter num2: ");
io::stdin().read_line(&mut num2).expect("Fail to read information");

let data1: i16 = num1.trim().parse().expect("Please enter a valid number");
let data2: u8 = num2.trim().parse().expect("Please enter a valid number");
println!("Result 1: {}, result 2: {}", data1, data2);

let mut res: i16 = data1 + data2 as i16;
println!("Result: {}", res);

}
