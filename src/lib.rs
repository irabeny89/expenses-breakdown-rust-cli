pub mod utilities {}

pub mod models {
    pub struct Expense {
        pub title: String,
        pub description: String,
        pub cost: f64,
    }

    impl Expense {
        pub fn new(title: String, description: String, cost: f64) -> Self {
            Self {
                title,
                description,
                cost,
            }
        }
    }

    pub struct Breakdown {
        pub title: String,
        pub description: String,
        pub gross: f64,
        pub balance: f64,
        pub total_cost: f64,
        pub expenses: Vec<Expense>,
    }

    impl Breakdown {
        pub fn new(title: String, description: String, gross: f64) -> Self {
            Self {
                title,
                description,
                gross,
                balance: 0.0,
                total_cost: 0.0,
                expenses: Vec::new(),
            }
        }

        pub fn show_breakdown(&self) -> () {
            let mut serial_number = 1;

            if self.expenses.is_empty() {
                println!("No expenses yet");
            }
            for expense in &self.expenses {
                println!("({})-----------------", serial_number);
                println!("{} - {}", expense.title, expense.cost);
                println!("{}", expense.description);
                println!("-----------------({})", serial_number);
                serial_number += 1;
            }

            println!("\n>>{}", self.title);
            println!(">{}", self.description);
            println!("--------Breakdown---------");
            println!("Total expenses:\r\n{}", self.expenses.len());
            println!("Gross:\r\n{}", self.gross);
            println!("Balance:\r\n{}", self.gross - self.total_cost);
            println!("Total cost:\r\n{}", self.total_cost);
            println!("--------Breakdown---------\n");
        }

        pub fn add_expense(&mut self, expense: Expense) {
            self.total_cost += expense.cost;
            self.balance = self.gross - self.total_cost;
            self.expenses.push(expense);

            println!("Expenses count: {}", self.expenses.len());
        }
    }
}
