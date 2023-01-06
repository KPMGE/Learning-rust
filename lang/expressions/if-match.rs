// rust is an expressions language, almost everything can potentially 
// be interpreted as an expression and be stored in a variable. When using 
// expressions and specially when working with Results and enums, we use 
// match and if let. They are almost the same thing, but if let is a shorthand
// for a match in which we just care about a specific case.

fn return_none() -> Option<i32> {
  // we don't need the return statement in here, 
  // if we don't put a ; at the end of the last expression 
  // in a function, that's gonna be the return of such a function,
  // that's a pretty common pattern in rust, usually we just use return
  // when we wanna return early
  None
}

fn return_some_num() -> Option<i32> {
  Some(10)
}

fn main() {
  let n = 10;

  match n {
    1 => println!("the number is 1"),
    2 => println!("the number is 2"),
    3 => println!("the number is 3"),
    _ => println!("the number is not 1, nor 2 neither 3"),
  }

  match n {
    0..10 => println!("the number is between 1 and 10"),
    _ => println!("the number is not between 1 and 10"),
  }

  match return_none() {
    Some(n) => println!("n: {}", n),
    None => println!("None"),
  }

  // this is a shorthand for a match statement, we just care 
  // about the Some variant of the Option enum.
  if let Some(n) = return_some_num() {
    println!("n, inside if let: {}", n);
  }
}
