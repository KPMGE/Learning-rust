// when defining traits, a really interesting feature is that
// we can define an associated type that will be replaced when the trait implementer 
// implments the trait. This shines whenever we need the trait to do more than just 
// describe methods. Actually, the Iterator trait is defined like this: 

pub trait Iterator {
    // associated type
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // ...
}

// when implementing this trait, we must specify the associated type
// for example, the args function, that returns an iterator is defined like this: 
impl Iterator for Args {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        // ...
    }
}

// we can access the associated type just like we access the static functions, with ::
fn collect_into_vector<I: Iterator>(iter: I) -> Vec<I::Item> {
    let mut results = Vec::new();
    for value in iter {
        results.push(value);
    }
    results
}

// in the example above, the only way to access the datatype for the return vector is by 
// using the I::Item, cuz if we otherwise had used Vec<I>, we'd be claiming to return a vector
// of iterators, which is of course not what we desire.

// when creating generic code, we can force the associated type to implement traits too, or even,
// we can create brand new traits which are subsets of a more generic trait by pre-specifying the 
// associated type.

fn dump<I>(iter: I) 
// the generic parameter I must implement the Iterator trait and its associated type Item 
// must implement the Debug trait so we can print it using {:?}
where I: Iterator, I::Item: Debug {
    // ...
}

// the example above works for any iterator that satisfies the bounds, we can stricter by creating
// a new iterator specifying the Item. So Iterator<Item=String> is itself a new trait, but it's a
// trait only for iterators that iterate over strings and as String implements the Debug trait, we
// can simply define

fn dump<I>(iter: I)
where I: Iterator<Item=String> {
    for (idx, str) in iter.enumerate() {
        println!("{}: {:?}", idx, str);
    }
}
