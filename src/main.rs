fn main() {

  // Loop and break him
  let mut counter = 0;
  loop {
      println!("Counter: {}", counter);
      counter += 1;

      if counter == 5 {
          break; 
      }
  }

  //Cycle While
  let mut number = 3;
  while number > 0{
    println!("{}", number);
    number -= 1;
  }
  println!("LiftOff");

  //Cycle for
  for number in 1..5 {
    println!("Number: {}", number);
  }

  //work with Break and Continue for cycles
  for number in 1..10 {
    if number % 2 == 0{
      continue;
    }
    if number > 7{
      break;
    }
    println!("Odd number: {}", number);
  }
  // Loop cycle 10 to 1
  let mut num = 10;
  loop {
    if num >=1 {
      println!("Number: {}", num);
      num -= 1;
    } else {
    println!("Liftoff!");
    break;  
    }
    }
  
  // calculate all numbers from 1 to 10
  let mut x = 1;
  let mut y = 0;
  while x<=10 {
    y = x + y;
    println!("Addition: {}",y);
    x += 1;
  }
  println!("Final addition: {}", y);

  //print all numbers 1-10 without %3
    for x in 1..=10 {
    if x % 3 != 0 {
  println!("Number: {}", x);
  } else {
      continue
  }
  }
  
  
  
}