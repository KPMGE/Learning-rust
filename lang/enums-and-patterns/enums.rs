// enums in rust are really powerful, they are used when we wanna represent
// an entity that can be either one thing or another. What makes enums so special
// is that, they can carry data with them, which makes them quite flexible. Besides that, 
// we can even implement methods for structs the same way we do for normal structs
//
// NOTE: rust stores enums in memory using a tag field that can be up to 8 bytes, but in some cases
// the rust compiler can already optimize to get rid of that tag, specifically, we should consider
// wrap our data into a 'Box' as soon as the size of that field becomes relevant, cuz the boxed
// value is just a pointer to some heap-allocated data and so, the compiler can optimize the code
// for better memory usage.

// if we don't specify, the enums hold values starting from 0
enum HttStatus {
  Ok = 200,
  NotFound = 404,
  // ...
}

// structs can hold data, and that becomes really useful when modeling tree-like datastructures
enum Json {
  Null, 
  Boolean(bool),
  Number(f64),
  String(String),
  Array(Vec<Json>),
  Object(Box<HashMap<String, Json>>) // the box ensures that we don't copy the whole HashMap
}
