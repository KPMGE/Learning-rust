// tuples in rust are much like python, usually we use them as a way of returning more
// than one thing from a function or as a small datastructure.
//
// NOTE: we can acess the elements of a tuple only by using 'tuple.1, tuple.2 ...' we can't use 
// an index, just like in an array. Tuples can have any type associated with them.
//
// NOTE: we can use destructuring just like in javascript when working with tuples


fn main() {
  // some text, &str
  let text = "hello world";
  // the split_at function returns a tuple: (&str, &str), we then 
  // destructure it and create two variables head and tail, just like
  // we do in javascript.
  let (head, tail) = text.split_at(6);

  // the above is the same as: 
  // let temp = text.split_at(6);
  // let head = temp.0;
  // let tail = temp.1;

  println!("head: {}", head);
  println!("tail: {}", tail);
}
