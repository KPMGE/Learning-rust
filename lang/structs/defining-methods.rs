// we can define methods for structs or other types in rust. In contrast to 
// other languagues like C++ and java, in rust we define all methods for a specific
// value in a separate 'impl' block. This is good cuz we have consistency, defining methods 
// for structs, enums and traits follows the same pattern.
//
// NOTE: rust calls the functions with whatever type we're calling the function with the 'self'
// parameter.
//
// NOTE: if our function doesn't need to change or access the underlying type, we can create a
// 'static' method. It's just a method that does not accept the 'self' parameter and can be called
// directly from the type using ::. This one is commonly used to crate constructors.
//
// NOTE: the 'Self' refers to the type we're implementing methods to and can be used as a
// shorthand.

use std::mem::swap;

struct Queue {
  older: Vec<char>,
  younger: Vec<char>
}

impl Queue {
  fn new() -> Self {
    Queue { older: Vec::new(), younger: Vec::new() }
  }

  fn push(&mut self, c: char) {
    self.younger.push(c);
  }

  fn pop(&mut self) -> Option<char> {
    if self.older.is_empty() {
      if self.younger.is_empty() {
        return None;
      }
      swap(&mut self.older, &mut self.younger);
      self.older.reverse();
    }

    self.older.pop()
  }

  fn is_empty(&self) -> bool {
    self.younger.is_empty() && self.older.is_empty()
  }

  fn split(self) -> (Vec<char>, Vec<char>) {
    (self.older, self.younger)
  }
}

fn main() {
  let mut q = Queue::new();

  q.push('a');
  q.push('b');
  q.push('c');
  q.push('d');
  q.push('e');
  q.push('f');

  for c in 0..3 {
    if let Some(ch) = q.pop() {
      println!("character: {}", ch);
    }
  }

  let (older, younger) = q.split();

  println!("\nolder:");
  for c in older {
    print!("{} ", c);
  }

  println!("\n\nyounger: ");
  for c in younger {
    print!("{} ", c);
  }
}
