// in rust, we have a lot of ways to represent sequential data, namely, vectors, slices and arrays.
// arrays are fixex length and can't grow, vectors are heap-allocated and growable, slices are used 
// just like pointers to the first element of the array in C and they also carry how many elements 
// you got on that collection.
//
// NOTE: a &[T] is commonly used when we wanna accept an array or vector in a function.
// NOTE: a &mut [T] is the same as &[T], but we can mutate the entries

fn main() {
  // fixed-size array of i32 values
  let ints: [i32; 3] = [1, 2, 3];
  // we can initialize an array with a specific value.
  let zeros = [0; 10];

  println!("ints: {:?}", ints);
  println!("zeros: {:?}", zeros);

  // we can create a vector with some default values using the vec! macro
  let v1 = vec![1, 2, 3];
  // we could have used the Vec::new() function and them pushed the values we wanted too
  let mut v2 = Vec::new();
  v2.push(1);
  v2.push(2);
  v2.push(3);
  // another option would be use an iterator and the collect function, we need to 
  // define the type of v3 explicitly in this case.
  let v3: Vec<i32> = (0..3).collect();

  println!("v1: {:?}", v1);
  println!("v2: {:?}", v2);
  println!("v3: {:?}", v3);

  display_values(&v1);
  display_values(&ints);
}

// this function accepts any type of iterable, it can be an array or vector.
fn display_values(val: &[i32]) {
  println!("values: ");
  for v in val {
    println!("{}", v);
  }
}
