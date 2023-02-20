// sometimes, we have a immutable struct, but we want a little bit of 
// mutability, perhaps, we don't wanna deal with all the complexity of
// borrowing a mutable reference, as we just need mutability for a specific
// field of our struct. For those cases, we can use some of the Rust's special 
// types that allows us have that flexibility.

// the 'Cell' type allow us have a 'get' and 'set' methods even though we don't 
// have a mutable or shared reference to the 'Cell' itself.

// in this case, it can be overwelming to borrow a mutable reference to a 
// BankAccount if the only thing we want is increment the counter, so we can use
// a cell for that.
use std::cell::{Cell, RefCell};

#[derive(Debug)]
struct BankAccount {
  owner_name: String,
  // other private fields ...
  access_counter: Cell<i32>,
  bank_name: RefCell<String>
}

// Cells are good, but they don't support borrowing references, if we need that, 
// the right tool is RefCell, which works just like Cell, but with suport for borrowing.

fn main() {
  // create an immutable BankAccount struct
  let b = BankAccount {
    owner_name: "kevin".to_string(), 
    access_counter: Cell::new(0),
    bank_name: RefCell::new("some bank name".to_string()), 
  };

  println!("{:?}", b);

  // we can set the value, even though our BankAccount is a immutable struct 
  b.access_counter.set(100);

  // we can borrow a reference to a bank_name
  let b_ref = b.bank_name.borrow();
  println!("b_ref: {:?}", b_ref);

  println!("{:?}", b);
}

