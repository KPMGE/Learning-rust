// in rust, there are 2 ways of writing polymorphic code, using trait objects 
// and using generics, a trait object is a pointer to some type that implements some
// specific trait. One important thing to notice in here is that, as rust needs to know 
// what methods it should call on the referent pointer, it stores some information about 
// the referents type.
//
// NOTE: we can't have a variable with a trait type, but we can have a reference

use std::io::Write;

fn main() {
    let mut buf = vec![];
    // let writer: Write = buf;   - this does not work, we can't have a variable with type Write,
    // which is a trait.

    let writer: &mut Write = &mut buf; // this works, we can have a reference to any value that
    // implements the Write trait.
}
