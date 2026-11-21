fn main() {

    let (name, department, mut salary): (String, Departments, u16)= ("Ayhan Bilici".to_string(), Departments::Marketing, 6000);

    //name = "Hakan Bilici".to_string(); // Error! -> immutable

    salary += 700;

    println!("Name: {name} - department: {department:?} - salary: {salary}"); // Name: Ayhan Bilici - department: Marketing - salary: 6700
}

#[derive(Debug)]
enum Departments {

    FinTech,
    Finance,
    Sales,
    Operations,
    Marketing,
    HR,
    Audit
}
