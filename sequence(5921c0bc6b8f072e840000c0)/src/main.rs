fn main() {
    println!("{:?}", sequence_classifier(&[3, 5, 8, 9, 14, 23]));
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Order {
    Unordered,
    Increasing,
    NotDecreasing,
    Decreasing,
    NotIncreasing,
    Constant,
}

// use preloaded::Order;

fn sequence_classifier(arr: &[i32]) -> Order {
    let mut possible_orders = vec![
        Order::Constant,
        Order::Increasing,
        Order::NotDecreasing,
        Order::Decreasing,
        Order::NotIncreasing,        
    ];
    for w in arr.windows(2) {
        let a = w[0];
        let b = w[1];
        if a > b {
            possible_orders.retain(|&o| ![Order::Increasing, Order::NotDecreasing, Order::Constant].contains(&o));
        } else if a < b {
            possible_orders.retain(|&o| ![Order::Decreasing, Order::NotIncreasing, Order::Constant].contains(&o));
        } else {
            possible_orders.retain(|&o| ![Order::Increasing, Order::Decreasing].contains(&o));
        }
        if possible_orders.is_empty() {
            return Order::Unordered;
        } else if possible_orders.len() == 1 {
            return possible_orders[0];
        }
    }
    possible_orders[0]
}
