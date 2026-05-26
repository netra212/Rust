#[cfg(test)]
mod setup;

enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
}

fn process_order_status(_status: OrderStatus) -> String {
    // TODO: Use pattern matching to return appropriate status messages
    match _status {
        OrderStatus::Pending => String::from("Order status Pending"),
        OrderStatus::Processing => String::from("Order status Processing"),
        OrderStatus::Shipped => String::from("Order status Shipped"),
        OrderStatus::Delivered => String::from("Order status Delivered"),
    }
}

fn categorize_status_type(_status: &OrderStatus) -> String {
    // TODO: Group statuses into categories: "active", "in_transit", "complete"
    match _status {
        OrderStatus::Pending => String::from("active"),
        OrderStatus::Shipped => String::from("in_transit"),
        _ => String::from("complete"),
    }
}

fn check_status_transition(_from: &OrderStatus, _to: &OrderStatus) -> bool {
    // TODO: Return true if the status transition is valid (Pending->Processing->Shipped->Delivered)
    match (_from, _to) {
        (&OrderStatus::Pending, &OrderStatus::Processing) => true,
        (&OrderStatus::Processing, &OrderStatus::Shipped) => true,
        (&OrderStatus::Shipped, &OrderStatus::Delivered) => true,
        _ => false,
    }
}

fn count_orders_by_status(_orders: &[OrderStatus]) -> (u32, u32, u32, u32) {
    // TODO: Count orders in each status, return (pending, processing, shipped, delivered)
    let mut pending = 0;
    let mut processing = 0;
    let mut shipped = 0;
    let mut delivered = 0;

    for order in _orders {
        match order {
            OrderStatus::Pending => pending += 1,
            OrderStatus::Processing => pending += 1,
            OrderStatus::Shipped => pending += 1,
            OrderStatus::Delivered => pending += 1,
        }
    }

    (pending, processing, shipped, delivered)
}

fn main() {
    println!("=== Order Status Enum Matching ===");

    // Test your implementations
    let pending = OrderStatus::Pending;
    let processing = OrderStatus::Processing;
    let _shipped = OrderStatus::Shipped;
    let _delivered = OrderStatus::Delivered;

    println!("Status: {}", process_order_status(pending));
    println!("Category: {}", categorize_status_type(&processing));

    println!(
        "Can transition Pending->Processing? {}",
        check_status_transition(&OrderStatus::Pending, &OrderStatus::Processing)
    );
    println!(
        "Can transition Shipped->Pending? {}",
        check_status_transition(&OrderStatus::Shipped, &OrderStatus::Pending)
    );

    let orders = vec![
        OrderStatus::Pending,
        OrderStatus::Processing,
        OrderStatus::Pending,
        OrderStatus::Shipped,
        OrderStatus::Delivered,
    ];

    let counts = count_orders_by_status(&orders);
    println!("Order counts: {:?}", counts);
}

// ============================================================
// 💡 HINTS (read only if stuck)
// ============================================================
//
// process_order_status:
//   - Use a match expression with each enum variant
//   - Return a descriptive string for each variant
//   - Look up: match, enum pattern matching
//
// categorize_status_type:
//   - Match on the enum reference; group Pending and Processing together
//   - Look up: matching on references, multiple patterns with |
//
// check_status_transition:
//   - Match on the (from, to) tuple; only allow sequential forward moves
//   - Look up: tuple patterns in match
//
// count_orders_by_status:
//   - Iterate and count with a mutable counter for each variant
//   - Or use filter().count() for each variant
//   - Look up: iter(), filter(), match inside closures
