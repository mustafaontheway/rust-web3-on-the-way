fn main() {

}

struct FamilyInfo {

    is_married: bool,
    num_of_children: u8,
    has_a_house: bool
}

enum Departments {
    
    FinTech,
    Sales,
    Marketing,
    Finance,
    Operations,
    Audit,
    HR
}

struct Employee<'a> {

    full_name: &'a str,
    department: Departments,
    salary_usd: u16,
    about: FamilyInfo
}
