// Borrow and Slice
// Borrow walk along with owner means if i changed anythin on the borrow side then owner will know immedately.
// Slice

struct Book {
    book_name: String, // We use an owned String, because each User
    author_name: String,
    page_count: u64,
    book_price: u64,
    book_sales_count: u64,
}

struct Rectangle{
    width: i32, 
    height: i32
}

fn cal_area(rec: Rectangle) -> i32{
    rec.height * rec.width;
}

fn main() {
    let book1 = Book{
        book_name: String::from("How to wins and influence people."),
        author_name: String::from("Dale Carnegi"),
        page_count: 234,
        book_price: 300,
        book_sales_count: 1245
    }

    let book2 = Book{
        book_name: String::from("Rich Dad Poor Dad"),
        author_name: String::from("Rober"),
        page_count: 301,
        book_price: 250,
        book_sales_count: 4502
    }

    let books = vec!(book1, book2);

    for book in &books{
        println!("{}",books[book]);
    }

    // Rectangle Initialization. 
    let r1 = Rectangle{
        width: 100, 
        height: 20
    }

    println!("{}", cal_area(&r1));
}


