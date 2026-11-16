fn main() {

    let names = ["Mustafa", "Ayhan", "Bengü"];

    let mut new_names: Vec<String> = Vec::new();

    for name in names {

        let updated_name = format!("Dear {}", name);

        new_names.push(updated_name);
    }

    println!("{new_names:?}")
}

// ["Dear Mustafa", "Dear Ayhan", "Dear Bengü"]
