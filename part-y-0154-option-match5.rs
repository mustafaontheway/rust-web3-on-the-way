fn main() {

    let s1 = secure_sum(Some(-5000), Some(12));

    println!("Sum 1: {s1:?}");

    let s2 = secure_sum(Some(-5000), None);

    println!("Sum 2: {s2:?}");
}

fn secure_sum(x: Option<i128>, y: Option<i128>) -> Option<i128> {

    match (x, y) {
        
        (Some(x), Some(y)) => Some(x + y),
        _ => None
    } 
}

// Sum 1: Some(-4988)
// Sum 2: None
