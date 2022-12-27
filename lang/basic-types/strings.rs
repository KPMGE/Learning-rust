// in rust we got two types of strings, the &str, which is a slice of str. 
// There is also the String type, which is actually implemented as a vector. 
// the main difference between them is that, a &str represents a heap-allocated 
// fixed-size string, while a String is a heap-allocated resizeable string.

// NOTE: usually when passing in strings to functions, we use &str, but when defining 
// fields of a struct, we use a String.

// NOTE: primitive types, like str implement the Copy trait by default, so we can create 
// a lot of copies from them without worring about ownership. The problem is that, when we 
// use a String, it does not implement it by default, so when we pass in a String variable by
// value, we actually tranfer the ownership of such a variable, so that we get some compilation
// errors.

fn main() {
  // define a &str
  let a: &str = "kevin";
  // copy variable a to b
  let b = a;
  // print them out
  println!("a: {}", a);
  println!("b: {}", b);

  // create a String variable
  let c = String::from("kevin");
  // transfer ownership of c to d
  let d = c;
  // if we try to print c, the compiler wont let us compile the code, cuz the ownership
  // of c was moved to d, so we don't know what could have happend to it, so it's not safe using it.
  // println!("c: {}", c);
  println!("d: {}", d);
}
