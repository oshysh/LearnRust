fn main() {
  // IF construction
  let number = 10;

  if number > 5 {
      println!("The number is greater than 5.");
  }

  //IF-ELSE construction
  let number = 10;

  if number > 15 {
      println!("The number is greater than 15.");
  } else if number == 10 {
      println!("The number is exactly 10.");
  } else {
      println!("The number is less than 10.");
  }
  //Don't understand
  let condition = true;
  let number = if condition { 5 } else { 10 }; 

  println!("The value of number is: {}", number);
  
  // comparing temperature
  let temperature  = 25;
  
  if temperature >=30 {
    println!("It's hot outside!");
    } else 
    if temperature >= 15 {
      println!("The weather is nice.");
    } else {
      println!("It's cold outside!");
    }
  let is_raising = false;
  let temperature = if is_raising {"Take an umbrella!"} else {""};
  println!("{}", temperature);
  
  
  
       
  
}