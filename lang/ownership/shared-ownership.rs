// in some situations, we wanna share the same entity throughout a lot of different functions 
// or something like that, but we don't wanna give ownership to the function or variable when we 
// do that nor we wanna copy over the whole structure. We can use the Rc and Arc types for that, 
// they do the same thing, they hold a pointer to some data allocated on the stack along with a
// counter, when we request a clone, we don't copy the whole thing, instead, we copy only the
// pointer and increment the counter, so that, we can have as many copies as we want, when the
// variables go out of scope, the counter is decremented, so that we know when it's time to free 
// the heap-allocated value. The difference between Rc an Arc is that, Rc works just for
// single-threaded programs and it's faster than Arc, which is meant for multi-threaded
// environments.
//
// NOTE: as the pointer is shared in a Rc or Arc type, the data is immutable to avoid undefined
// behavior.

use std::rc::Rc;

fn main() {
  // we create a new Rc<String>, the Rc got a pointer to a heap-allocated string
  let p1 = Rc::new(String::from("kevin"));
  // we copy the pointer and increment the counter
  let p2 = p1.clone();
  // we copy the pointer and increment the counter
  let p3 = p1.clone();

  println!("p1: {:?}", p1);
  println!("p2: {:?}", p2);
  println!("p3: {:?}", p3);
}
