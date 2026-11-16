fn main() {

    let sales_by_emps = vec![("ayhan", 32500u16), ("hakan", 24000), ("bengü", 32400)];

    let mut names: Vec<&str> = Vec::new();

    let mut sales: Vec<u16> = Vec::new();

    for (name, s) in &sales_by_emps {

        names.push(*name);
        sales.push(*s);
    }

    println!("Names list: {names:?}");

    println!("Sales list: {sales:?}");
}

// Names list: ["ayhan", "hakan", "bengü"]
// Sales list: [32500, 24000, 32400]
