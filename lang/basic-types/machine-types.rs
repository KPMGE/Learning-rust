// NOTE: we can cast types using the 'as' keyword.
// NOTE: usually, rust can infer most types for us, but sometimes
// we need to make sure we're using the right ones, we can do that by 
// simply placing the type along with its value.
// NOTE: types have traits that we can use, they act like methods on classes

fn main() {
  // signed 32-bit integer value
  let a = 3i32;
  // same as above, rust can infer the type for us
  let b = 3;
  // same as b, but converted in a floating-point 64-bit number
  let c = b as f64;

  println!("a: {}", a);
  println!("b: {}", b);
  println!("sqrt(c): {}", f64::sqrt(c));
}
