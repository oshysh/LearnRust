fn main() {
   name("Oks");
   //let a = "OKS";
   //println!("Hello, {}", a);
   let mut x = 10;
   let x = 20;
   let y = x + 20;
   println!("{}", y);
   let num = 20;
   if num > 0 {
   println!("{} - Positive number", num);
    } else {
    if num == 0 {
    println!("{} - Zero", num);
    
    } else {
    println!("{} - Negative number", num);
    }
    }
    if num > 10 {
    if num % 2 == 0{
    println!("{} - greater than 10 and even", num);
    }
    }
    let mut num1 = 1;
    loop {
    if num1 <= 10{ 
    println!("{}- cycle loop" , num1);
    num1 += 1; 
    } else {
    break;
    }
    }
    let mut num2 = 1;     
    while num2<=10 {
    println!("{} - cycle while", num2);
    num2 += 1;
    }
    for i in 1..= 10 {
    println!("{} - cycle for", i);
    }
    print_message("Hello, ownership!");
    let sq = square(2, 4);
    println!("{}", sq);
    let res = compare (10, 20);
    println!("Result - {}", res);
    let number = 5;
    let result = factorial(number);
    println!("{}! is {}", number, result); 
    let op =  calculate (10, 20, "add");
    println!("{}", op);


}

fn name (name: &str){
println!("Hello {}! Welcome to programming", name)
}
fn print_message (message: &str){
println!("{}", message)
}
fn square (a: i32, b: i32) -> i32 {
a * b
}
fn compare (c: i32, d: i32) -> String {
if c > d {
String :: from("greater")
} else 
if c < d {
String::from("less")
}
else {
String::from("equal")
}
}
fn factorial(a: u32) -> u32 {
let mut res = 1;
let mut i = a;
while i >= 1 {
res *= i;
i -= 1;
}
res
}
fn calculate (q: i32, l: i32, operation: &str) -> String
{
if operation == "add" {
let m = q + l;
format!("Add is {}", m)
} else if operation == "subtract"{
let m = q - l;
format!("Subtract is {}", m)
}else if operation == "multiply"{
let m = q * l;
format!("Multiply is {}", m)
}else if operation == "divide"{
let m = q / l;
format!("Divide is {}", m)
}else{
String::from("Unknown")
}
}

