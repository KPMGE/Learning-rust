// in rust, references are a central point. They are much like C or C++ pointers, 
// but in safe rust, there is no way of passing a null pointer or something like this. 
// Just like in c, the expression '&a' gets the address of a in memory, and '*a' gets the 
// value a is pointing to, unlike C, references in rust are immutable by default, and if we 
// want a mutable one, we must explicitly mark the reference as mutable by using the keyword 'mut'.

fn read_num(num: &i32) {
  println!("i can read the value from num, but can't change it: {}", num);
}
fn change_num(num: &mut i32) {
  println!("i can read and change the value of num: {}", num);
  *num = 200;
}

fn main() {
  // mutable i32 variable
  let mut a = 10;
  read_num(&a);
  change_num(&mut a);

  println!("num after: {}", a);
}
