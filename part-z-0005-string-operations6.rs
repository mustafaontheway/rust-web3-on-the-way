fn main() {

    let mut her_name = String::from("Aygün Arslan");

    println!("{}", greeting(&mut her_name))
}

fn greeting(user_name: &mut String) -> String {

    user_name.push_str(", our dear customer!");

    let final_greeting = format!("Welcome, {}", user_name);

    final_greeting
}

// Welcome, Aygün Arslan, our dear customer!
