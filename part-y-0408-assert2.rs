fn main() {

    let mut emp_ayhan_salary = 4200u16;

    emp_ayhan_salary = salary_inc(emp_ayhan_salary, 3000);

    emp_ayhan_salary = salary_dec(emp_ayhan_salary, 1400);

    println!("{emp_ayhan_salary}") // 5800
   
}

fn salary_inc(mut current_salary: u16, amount: u16) -> u16 {

    current_salary += amount;

    assert!(current_salary > 1500 && current_salary < 7500, "Consider our firm's salary policy!");

    current_salary
}

fn salary_dec(mut current_salary: u16, amount: u16) -> u16 {

    current_salary -= amount;

    assert!(current_salary > 1500 && current_salary < 7500, "Consider our firm's salary policy!");

    current_salary
}
