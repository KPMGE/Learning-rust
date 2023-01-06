// in rust, the way we manage errors is through the Result enum, it has 2 
// variants, Ok and Err, basically then, if an error has occurred, we wrap it
// with the Err variant, otherwise we wrap it with the Ok one. Later, we can use
// a match or if statment to check if an error has occurred or not and then deal with it.
// sometimes, though, we don't wanna deal with the error, we want it do be send up the stack
// in this situation, we can use the ? at the end of the function that returns the result, 
// it will check if the Err variant has occurred and, if so, it will return early with that 
// variant, otherwise it unwraps the Ok variant, so its value becomes available to be used.


// NOTE: sometimes, many things can go wrong and they got different error types, 
// we can define a generic Error and a generic Result as: 

type GenErr = Box<dyn std::error::Error>;
type GenResut<T> = Result<T, GenErr>;

fn parse_num(num: &str) -> Result<u8, std::num::ParseIntError> {
  num.parse::<u8>()
}

fn sum_nums(num1: &str, num2: &str) -> Result<u8, std::num::ParseIntError> {
  let n1 = num1.parse::<u8>()?;
  let n2 = num2.parse::<u8>()?;
  Ok(n1 + n2)
}

fn sum_nums_gen(num1: &str, num2: &str) -> GenResut<u8> {
  let n1 = num1.parse::<u8>()?;
  let n2 = num2.parse::<u8>()?;
  Ok(n1 + n2)
}

fn main() {
  // string with a number
  let num = "10";
  // tries to parse a number, which can fail and return a Result
  match parse_num(num) {
    Ok(n) => println!("The number parsed is: {}", n),
    Err(e) => eprintln!("an error has occurred: {:?}", e),
  }

  println!("unwrap_or: {}", parse_num(num).unwrap_or(0));

  match sum_nums("10", "10") {
    Ok(n) => println!("sum: {}", n),
    Err(e) => eprintln!("an error has occurred: {:?}", e),
  }

  match sum_nums_gen("10", "10") {
    Ok(n) => println!("sum gen: {}", n),
    Err(e) => eprintln!("an error has occurred: {:?}", e),
  }
}
