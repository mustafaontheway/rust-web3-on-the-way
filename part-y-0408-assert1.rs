fn main() {

    let mut finance_department_budget = Budget::new(23_000);

    finance_department_budget.budget_inc(95_000);

    finance_department_budget.budget_dec(100_000);
}

struct Budget(u64);

impl Budget {

    fn new(budget: u64) -> Self {

        assert!(budget > 20_000 && budget < 1_000_000, "Consider budget constraints!");

        Budget(budget)
    }

    fn budget_inc(&mut self, amount: u64) {

        let new_budget = self.0.saturating_add(amount);

        assert!(new_budget < 1_000_000, "Budget upper limit exceeded!");

        self.0 = new_budget;
    }

    fn budget_dec(&mut self, amount: u64) {

        let new_budget = self.0.saturating_sub(amount);

        assert!(new_budget > 20_000, "Budget lower limit exceeded!");

        self.0 = new_budget;
    }
}
