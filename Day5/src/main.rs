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

    let fruits = ["Apple", "Banana", "Mango"];

    let uppercase_fruits = fruits.iter().map(|&fruit| fruit.to_uppercase());

    println!("Uppercase fruits:");

    for uppercase_fruit in uppercase_fruits {
        println!("{uppercase_fruit}");
    }

    let nuts = ["ALMOND", "PEANUTS"];

    let fruits_and_nuts = fruits.iter().chain(&nuts);
    for f in fruits_and_nuts {
        println!("Aggregated foods: {f}");
    }

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let zipped: Vec<_> = numbers.iter().zip(words.iter()).collect();

    for (number, word) in zipped {
        println!("{number}: {word}");
    }
}
