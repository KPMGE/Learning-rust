// we can 'borrow' a variable for a certain amount of time, that means we get ownership 
// over it and after, we give it back to the callee. That pattern is pretty common in rust, 
// usually we want to pass in a parameter to a function which, in turn, modify it. That function
// must get ownership over the variable and once the function is done, it must give it back to the 
// callee. We do that by using mutable refences in rust.

fn main() {
  // create a mutable string.
  let mut address = String::from("Street 1");

  println!("address before check: {}", address);

  // function that takes a mutable refence to address and modify it
  address_checker(&mut address);

  println!("address after check: {}", address);
}

fn address_checker(address: &mut String) {
  // push_str changes the address string
  address.push_str(", 1234 Kingston");
}
