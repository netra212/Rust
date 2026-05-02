fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let mut iter = numbers.iter();

    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
}

// pub trait Iterator {
//     type Item;
//     fn next(&mut self) --> Option<Self::Item>;
// }
