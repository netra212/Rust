fn main() {
    let numbers = [11, 1, 21, 3, 4, 5];
    let mut iter = numbers.iter();

    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());

    let iter = numbers.iter().map(|x| x * 2).filter(|x| x > &5);
    println!("Iter: {iter:?}");

    let result: Vec<_> = iter.collect();
    println!("result: {result:?}");
}

// pub trait Iterator {
//     type Item;
//     fn next(&mut self) --> Option<Self::Item>;
// }
