fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Cannot divide by zero".to_string());
    }

    Ok(a / b)
}

fn get_item(index: usize) -> i32 {
    let items = vec![10, 20, 30];
    items[index] // panic if index out of bounds. 
}

async fn fetch_data(id: u32) -> Result<String, String> {
    if id == 0 {
        return Err("Invalid ID".to_string());
    }

    if id == 999 {
        return Err("Not found".to_string());
    }

    Ok(format!("Data for ID {}", id))
}

#[tokio::main]
async fn main() {
    println!("10 / 2 = {:?}", divide(10, 2));
    println!("10 / 0 = {:?}", divide(10, 0));
    println!("-10 / 2 = {:?}", divide(-10, 2));
    println!("Valid index (1): {}", get_item(1));
    println!("Valid index (0): {}", get_item(0));
    println!("Valid index (2): {}", get_item(2));
    println!("\n Run `cargo test` to see the panic tests");

    println!("Fetch ID 1: {:?}", fetch_data(1).await);
    println!("Fetch ID 0: {:?}", fetch_data(0).await);
    println!("Fetch ID 1: {:?}", fetch_data(999).await);
    println!("\n Run `cargo test` to see the async tests results");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_division() {
        let result = divide(10, 2);
        assert_eq!(result, Ok(5));
    }

    #[test]
    fn test_division_by_zero() {
        let result = divide(10, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_negative_numbers() {
        let result = divide(-10, 2);
        assert_eq!(result, Ok(-5));
    }

    #[test]
    fn test_valid_index() {
        let result = get_item(1);
        assert_eq!(result, 20);
    }

    #[test]
    #[should_panic]
    fn test_invalid_index() {
        get_item(10);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_panic_message() {
        get_item(100);
    }

    // Aysnc test.
    #[tokio::test]
    async fn test_fetch_success() {
        let result = fetch_data(1).await;
        assert_eq!(result, Ok("Data for ID 1".to_string()()));
    }

    #[tokio::test]
    async fn test_fetch_invalid_id() {
        let result = fetch_data(0).await;
        assert_eq!(result, Err("Invalid ID 0".to_string()()));
    }

    #[tokio::test]
    async fn test_fetch_not_found() {
        let result = fetch_data(999).await;
        assert_eq!(result, Err("Not found".to_string()()));
    }
}
