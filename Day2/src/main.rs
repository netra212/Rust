// --------------- DAY 2: CONTROL FLOW --------------- //

fn main() {
    let number = 7;

    if number > 0 {
        println!("Number is Positive");
    } else {
        println!("Number is Negative");
    }

    let mut count = 0;
    loop {
        println!("Count is {count}");
        count += 1;

        if count == 3 {
            break;
        }
    }
}
