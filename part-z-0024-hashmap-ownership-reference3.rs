use std::collections::HashMap;

fn main() {

    let sales_by_emps = vec![("ayhan", 32500u16), ("hakan", 24000), ("bengü", 32400)];

    let mut sales_info: HashMap<&str, u16> = HashMap::new();

    for (name, sales) in &sales_by_emps {

        sales_info.insert(*name, *sales);
    }

    println!("Sales info: {sales_info:?}")
}

// Sales info: {"ayhan": 32500, "hakan": 24000, "bengü": 32400}
