fn main() {
    let mut numbers: Vec<i32> = Vec::new();
    println!("Numbers: {numbers:?}");

    numbers.push(4);
    numbers.push(2);
    numbers.push(5);
    println!("Numbers: {numbers:?}");

    let fruits = vec!["Banana", "Apple", "Orange"];
    println!("Numbers: {fruits:?}");

    let second_fruits = &fruits[1];
    println!("Accessing or Referencing second element from fruits: {second_fruits}");

    // let not_present_element = &fruits[4];
    // println!("Trying to Accessing not present element from fruits: {not_present_element}");

    match fruits.get(2) {
        Some(fruit) => println!("The fruit is: {fruit}"),
        None => println!("No fruits found at this index"),
    }

    let mut numbers1 = vec![1, 30, 4, 5, 6, 8];
    println!("Length: {}", numbers1.len());
    println!("Contains: {}", numbers1.contains(&30));
    println!("Contains: {}", numbers1.contains(&31));

    numbers1.clear();
    println!("All Elements are clear, so Length: {}", numbers1.len());
    println!("is_Empty: {}", numbers1.is_empty());
}
