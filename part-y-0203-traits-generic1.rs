use std::cmp::PartialOrd;

fn main() {

}

enum Departments {

    FinTech,
    Finance,
    Sales,
    Operations,
    Accounting,
    HR,
    Audit
}

trait NumericOperations<T> {

    fn find_max(&self) -> T;
    fn find_min(&self) -> T;
}

struct DepartmentBudgeting<T> {

    department: Departments,
    quarterly_budgets: Vec<T>
}

impl<T> NumericOperations<T> for DepartmentBudgeting<T> 
where 
    T: PartialOrd + Copy

{

    fn find_max(&self) -> T {
        
        let mut max_budget = self.quarterly_budgets[0];

        for qb in &self.quarterly_budgets {

            if *qb > max_budget {
                max_budget = *qb;
            }
        }

        max_budget
    }

    fn find_min(&self) -> T {
        
        let mut min_budget = self.quarterly_budgets[0];

        for qb in &self.quarterly_budgets {

            if *qb < min_budget {
                min_budget = *qb;
            }
        }

        min_budget
    }
}
