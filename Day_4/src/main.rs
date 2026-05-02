use std::collections::HashMap;

fn main() {
    // let mut numbers: Vec<i32> = Vec::new();
    // println!("Numbers: {numbers:?}");

    // numbers.push(4);
    // numbers.push(2);
    // numbers.push(5);
    // println!("Numbers: {numbers:?}");

    // let fruits = vec!["Banana", "Apple", "Orange"];
    // println!("Numbers: {fruits:?}");

    // let second_fruits = &fruits[1];
    // println!("Accessing or Referencing second element from fruits: {second_fruits}");

    // // let not_present_element = &fruits[4];
    // // println!("Trying to Accessing not present element from fruits: {not_present_element}");

    // match fruits.get(2) {
    //     Some(fruit) => println!("The fruit is: {fruit}"),
    //     None => println!("No fruits found at this index"),
    // }

    // let mut numbers1 = vec![1, 30, 4, 5, 6, 8];
    // println!("Length: {}", numbers1.len());
    // println!("Contains: {}", numbers1.contains(&30));
    // println!("Contains: {}", numbers1.contains(&31));

    // numbers1.clear();
    // println!("All Elements are clear, so Length: {}", numbers1.len());
    // println!("is_Empty: {}", numbers1.is_empty());

    // // Slices of Vectors.
    // let v: Vec<i32> = vec![1, 2, 3, 4];
    // // let s = &v[1..3];
    // let mut vec: Vec<i32> = Vec::with_capacity(6);
    // vec.push(1);
    // vec.push(2);
    // vec.push(5);

    // //
    // macro_rules! vec {
    //     ($($x:expr), +) => ({
    //         let mut v = Vec::new();
    //         $(v.push($x);) + v
    //     });
    // }

    // // let a2 = vec![1, 2, 3]; // <-- This vector got converted into below vector underthehood which is a macro which creates code automatically.
    // // let mut v1 = Vec::new();
    // // v1.push(1);
    // // v1.push(2);
    // // v1.push(5);

    // //
    // let mut foods = vec!["Cake", "Pizza", "Mango", "Rice"];
    // foods.push("Drago fruits");
    // println!("foods are: {foods:?}");

    // let last_foods = foods.pop();
    // println!("Popping foods from last: {last_foods:?}");

    // fruits.remove(1):
    // println!("foods are: {fruits:?}");

    // HashMaps.
    let mut books: HashMap<String, u32> = HashMap::new();
    println!("Books: {books:?}");

    // Inserting data into hashmap.
    books.insert("Harry Porter".into(), 1997); // .into() and .to_string() both can be used, and helps to store string : "Harry Porter" into heap not stack.
    books.insert("The Little Prince".into(), 1943);
    books.insert("The Billionaire Mindset".into(), 1943);
    books.insert("The Little Prince".into(), 1973);
    books.insert("Rich Dad Poor Dad".into(), 1981);
    books.insert("The Intelligent Investor".into(), 1993);
    books.insert("Stop Worrying and Start Living".into(), 1922);
    println!("Books: {books:#?}");

    books.remove("The Billionaire Mindset");
    println!("Books: {books:#?}");
}
