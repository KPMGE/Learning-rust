// when we have arrays or any type of indexed content, we got
// a problem, cuz if we try to take a value out of that array 
// and assign it to a variable, rust would need to somehow store 
// the information about the ownership about that specific location, 
// that's not an efficient way of working, so by default, rust does not
// enable moving ownership of a specific position in an array. If we want 
// to just read the value, we could borrow a reference instead of owning it, 
// but if we really want ownership, there are some options, the best one is 
// probably marking the field we want as a Result<>, so that, a None is a valid 
// type, we can borrow the value and then put None in there. That pattern is 
// so common in rust that the time has dedicated syntax for that. We can now 
// use the take() function for that purpuse.

#[derive(Debug)]
struct Person {
  name: Option<String>,
  age: u32
}

fn main() {
  let mut people = vec![
    Person{ name: Some(String::from("Kevin")), age: 21 },
    Person{ name: Some(String::from("Luana")), age: 20 },
    Person{ name: Some(String::from("Lara")), age: 25 },
    Person{ name: Some(String::from("Camila")), age: 24 },
  ];

  // this does not work, we're trying to have ownership over a position in a vector,
  // that's not allowed.
  // let first_person = people[0].name;

  // this works, we're not having ownership in here, we're just borrowing a reference to that
  // position
  let first_person_name = &people[0].name;

  match first_person_name {
    Some(name) => println!("name: {:?}", name),
    None => println!("No name"),
  }

  // if we wanna take that name out, we could use some options, we could use the std::mem::replace
  // function to take that value out and replace it with None, or we could use the take function,
  // that does really the exact same thing, but in a more readable way.
  let second_person_name = std::mem::replace(&mut people[1].name, None);

  match second_person_name {
    Some(name) => println!("name: {:?}", name),
    None => println!("No name"),
  }

  let third_person_name = people[2].name.take();

  match third_person_name {
    Some(name) => println!("name: {:?}", name),
    None => println!("No name"),
  }
}
