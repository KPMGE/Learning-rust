// in rust, we can create generic structs, just like in c++. 
// actually, a lot of types in rust are made this way, like Vec, for example.

// now our queue is generic, T is gonna be replaced by any valid rust type.
struct Queue<T> {
  older: Vec<T>,
  younger: Vec<T>
}

impl<T> Queue<T> {
  fn new() -> Self {
    Queue { older: Vec::new(), younger: Vec::new() }
  }

  fn push(&mut self, c: T) {
    self.younger.push(c);
  }
}

fn main() {
  // even though our queue is generic, we can use the new method without 
  // explicitly telling the queue's type and let rust figure it out by itself.
  let mut q = Queue::new(); 
  // when we define the type explicitly, rust will throw an error if we call the queue with 
  // the wrong type.
  let mut q2 = Queue::<i32>::new();

  q.push(10);     // oh, this looks like a Queue<i32>
  // q.push('a');    // this one looke like a Queue<char>
  // q.push("heyo"); // hold on, i bet this one is a Queue<&str>

  q2.push(10);    // this is fine, 10 is an i32
  // q2.push('a');   // wtf? what are u trying to do my dude?
}
