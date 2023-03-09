// another way of writing polymorphic code in rust is using generic functions, the idea is pretty
// simple, we can enforce that the type our function is gonna be applied to implements one or more
// traits, this allows us to describe exactly the behavior we need from those functions or types.

// besides the standard prelude, we need to explicitly import the traits we wanna use. this makes 
// naming conflicts very unlikely
use std::fmt::Debug;

trait Test {
    fn test() -> String;
}

// this function takes a type that implements the Debug trait
fn display_top_2_elements<T: Debug + Test>(v: &Vec<T>) {
    for i in 0..2 {
        println!("{:?}", v[i]);
        println!("{}", T::test());
    }
}

impl Test for i32 {
    fn test() -> String {
        "i'm an i32".to_string()
    }
}

// we can specify more than one bound using the + operator, or using the where keyword
// fn run_query<M: Mapper + Serialize, R: Reducer + Serialize>(data: &DataSet, map: M, reduce: R) -> Results {
//     // ...
// }

// fn run_query<M, R>(data: &DataSet, map: M, reduce: R) -> Results
//     where M: Mapper + Serialize,
//           R: Reducer + Serialize {

// }

fn main() {
    let v = vec![1, 2, 3];

    display_top_2_elements(&v);
}
