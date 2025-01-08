fn main() {
    greet();
    greet_user("Alice");
  let sum = add(5, 3);
  println!("The sum is: {}", sum);
  let sum1 = add(10, 20);
  println!("{}", sum1);
}
// example easy function
fn greet() {
    println!("Hello, Rust!");
}

//example function with argument
fn greet_user(name: &str) {
    println!("Hello {}", name);
} 

//example function with returning value
fn add(x: i32, y: i32) -> i32 {
  x + y
} 
