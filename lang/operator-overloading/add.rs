// in rust, overriding operators is as simple as implementing some traits from the
// standard library, more specifically, from std::ops

#[derive(Debug)]
struct Complex<T> {
    re: T,
    im: T
}

// for any type T, that implements the std::ops::Add trait
impl<T> std::ops::Add for Complex<T> 
    // restricts T to a type that can be added yielding another T
    where T: std::ops::Add<Output = T>
{
    // Self == Complex<T>
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
} 

fn main() {
    let a = Complex { re: 1, im: 13 };
    let b = Complex { re: 6, im: 3 };
    let c = a + b;

    println!("{:?}", c);
}
