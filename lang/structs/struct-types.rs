// rust has basically 3 types of structs, tuple-like structs, structs 
// with fields and blank structs.


// this is an example of a blank struct, it occupies no memory
// at all, but can be useful when working with traits.
struct BlankStruct;

// this is an example of a tuple-like struct, we can create such a struct
// much like we'd do for a tuple, but using the name before the tuple.
// They can be quite useful when defining wrappers around types that already
// exist to get stricter type checking.
struct TupleLike(usize, usize);

// for example, if we're dealing with ascii characters, we might define a struct like:
struct Ascii(Vec<u8>);
// defining such a struct is better than passing around Vec<u8>s and explaning what they mean
// through comments.

// finally, we got 'normal' structs, they are really the same as in C or C++
struct Person {
  name: String,
  age: u32,
}

fn main() {
  let _a = BlankStruct;
  let _b = Ascii(vec![1, 3, 41]);
  let _c = Person{ name: "kevin".to_string(), age: 21 };
}
