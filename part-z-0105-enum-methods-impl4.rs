fn main() {

    let mut jina_salary = 3200u16;

    let jina_salary_in_rate = Departments::Marketing.salary_inc_rate();

    jina_salary += ((jina_salary as f32) * jina_salary_in_rate) as u16;

    println!("New salary: {jina_salary}") // New salary: 3552
}

enum Departments {

    FinTech,
    Sales,
    Marketing,
    Operations,
    Accounting,
    Audit,
    HR
}

impl Departments {

    fn salary_inc_rate(&self) -> f32 {

        match self {
            
            Departments::FinTech | Departments::Sales => 0.12,
            Departments::Marketing => 0.11,
            Departments::Operations | Departments::Accounting => 0.10,
            Departments::Audit => 0.08,
            Departments::HR => 0.07
        }
    }
}

