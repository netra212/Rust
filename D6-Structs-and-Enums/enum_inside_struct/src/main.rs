#[cfg(test)]
mod setup;

enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
}

struct Order {
    order_id: u32,
    status: OrderStatus,
}

fn create_order_with_status(_id: u32, _status: OrderStatus) -> Order {
    // TODO: Create an Order struct containing the given status enum
    Order {
        order_id: _id,
        status: _status,
    }
}

fn update_order_status(_order: &mut Order, _new_status: OrderStatus) {
    // TODO: Update the status field of the order
    _order.status = _new_status;
}

fn filter_orders_by_status(_orders: &[Order], _target_status: OrderStatus) -> Vec<u32> {
    // TODO: Return order IDs that match the target status
    _orders
        .iter()
        .filter(|_orders| _orders.status == _target_status)
        .map(|_orders| _orders.order_id)
        .collect()
}

fn batch_process_orders(_orders: &mut [Order]) -> u32 {
    // TODO: Move all Pending orders to Processing, return count of updated orders
    let mut count = 0;

    for order in _orders.iter_mut() {
        match order.status {
            OrderStatus::Pending => {
                order.status = OrderStatus::Processing;
                count += 1;
            }
            _ => {}
        }
    }
    count
}

fn main() {
    println!("=== Complex Data Structures (Enum in Struct) ===");

    // Test your implementations
    let order1 = create_order_with_status(1001, OrderStatus::Pending);
    let mut order2 = create_order_with_status(1002, OrderStatus::Processing);

    println!("Created order {} with initial status", order1.order_id);

    update_order_status(&mut order2, OrderStatus::Shipped);
    println!("Updated order {} status", order2.order_id);

    let orders = vec![
        create_order_with_status(2001, OrderStatus::Pending),
        create_order_with_status(2002, OrderStatus::Processing),
        create_order_with_status(2003, OrderStatus::Pending),
        create_order_with_status(2004, OrderStatus::Delivered),
    ];

    let pending_ids = filter_orders_by_status(&orders, OrderStatus::Pending);
    println!("Pending order IDs: {:?}", pending_ids);

    let mut mutable_orders = vec![
        create_order_with_status(3001, OrderStatus::Pending),
        create_order_with_status(3002, OrderStatus::Pending),
        create_order_with_status(3003, OrderStatus::Processing),
    ];

    let updated_count = batch_process_orders(&mut mutable_orders);
    println!("Updated {} orders to processing", updated_count);
}

// ============================================================
// 💡 HINTS (read only if stuck)
// ============================================================
//
// create_order_with_status:
//   - Initialize Order with the provided id and status
//   - Look up: struct initialization with enum fields
//
// update_order_status:
//   - Assign the new_status to order.status
//   - Look up: mutable references, field assignment
//
// filter_orders_by_status:
//   - Iterate over orders and collect order_id for each order whose status matches
//   - Use match or == on enum variants
//   - Look up: iter(), filter(), map(), collect()
//
// batch_process_orders:
//   - Iterate mutably, find Pending orders, change them to Processing, and count the updates
//   - Look up: iter_mut(), match on mutable references, counting
