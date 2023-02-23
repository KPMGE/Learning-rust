// in rust, defining a trait is simple, we just give it a name and start listing the
// functions it has, just like any interface in c++ or java. The difference is that, in
// rust we can even define static methods, cuz as we know, static methods in rust are just
// methods that do not accept the self argument.
// Moreover, we can 'extend' traits, so every object that implements our trait must implement the
// associated trait as well.
//
// NOTE: In rust, when we use trait that are not on the standard prelude, we must import them
// NOTE: We can define a default implementation for the method when defining the trait, so the
// implementer of such a trait may or may not implement that method.
// NOTE: We can extend any type, even the standard ones, like str adding methods to them through
// traits.


// in rust we can invoke a method in 4 different ways, for example:
"hello".to_string(); // using the value
str::to_string("hello"); // using the type
ToString::to_string("hello"); // using the trait
<str as ToString>::to_string("hello"); // using a fully qualifier

use std::io::{Write, Result};

struct Sink;

// we just need to implement the write and flush methods, the write_all method has a default
// implementation.
impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        // Claim to have successfully written the whole buffer.
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

// we define our own trait with the functions we need
trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

// we implement that trait for a type, any type, even the standard char
impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        return true;
    }
}

trait StringSet {
    /// Return a new empty set. this is a static method
    fn new() -> Self;
    /// Return a set that contains all the strings in `strings`.
    fn from_slice(strings: &[&str]) -> Self;
    /// Find out if this set contains a particular `value`.
    fn contains(&self, string: &str) -> bool;
}
