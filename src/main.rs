fn main() {
    // learning of const
  const PI: f64 = 3.14;
  const SEC_IN_MIN: u32 = 60;
  println!("PI is {}", PI);
  println!("Seconds in one minute is {}", SEC_IN_MIN);

// learning shadowing const
  let x = 5;
   println!("Initial value of x: {}", x);

  let x = x + 1; 
  println!("Value of x after shadowing: {}", x);

  let x = "Now x is a string"; 
  println!("x is now: {}", x);

  // create new const
  const MAX_POINTS : u32 = 100;
  println!("{}",MAX_POINTS);

  //create new value and shadowing of it
  let score = 50;
  println!("{}", score);
  let score = score + 10;
  println!("{}", score);
  let score = format!("Your score is {}", score);
  println!("{}", score);
}