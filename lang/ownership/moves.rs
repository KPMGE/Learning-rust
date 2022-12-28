// moves are the essential way rust uses to implement its ownership rules. 
// basically, everytime we assign a complex value to a function, pass it 
// to a function or something like that, what we actually do is pass in 
// the ownership of such a variable to that function or variable, so that, 
// once the function or variable goes out of scope, rust automatically drops 
// the whole ownership tree related to that specific structure. That's because, 
// let's suppose you got a vector of structs, the vector owns its elements, and 
// each element, which is a struct, own its fields, that in turn can own other 
// stuff. That makes up a ownership tree. The whole point in here is that, the variable
//  or function that has ownership over the vector dictates when the whole thing is gonna 
//  be freed, that happens when we go out of scope for that variable or function.
//
//  NOTE: when we don't wanna give ownership to a function or variable, we must sign that 
//  explicitly by cloning the value, in rust primitive types implement type Copy and Clone traits 
//  by default, so when we pass in an i32, a &str or f64 for example, we actually copy the value 
//  to the function or variable, that's a safe way of working, cuz those types don't depend on
//  anything that changes, it's different when we use a complex type like a String, cuz we know
//  that type is modifiable, so if we hold a lot of references to the same thing on the heap, 
//  we could face a lot of problems cuz we wouldn't know who could change it, or even drop it.


fn main() {
  // create a string s, we know that a String is a complex type defined
  // as a Vec<u8> type. So that, the ownership comes into play in here.
  let s = String::from("hello world");
  // k now has ownership over the string, and s is left uninitialized
  let k = s;

  // that's gonna trigger a compile error, we're trying to use s, but we know s
  // has been uninitialized as the ownership over the string was moved to k
  // println!("s: {}", s);


  // if we want s to still hold the string even after assigning it to k, we need to explicitly
  // clone s. Like so: 

  let s2 = String::from("hello world 2");
  let k2 = s2.clone();
  println!("s2: {}", s2);
}
