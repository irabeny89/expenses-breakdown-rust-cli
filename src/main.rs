use expenses_breakdown_rust_cli::models::{Breakdown, Expense};
use std::io::stdin;

fn initialize_breakdown() -> Breakdown {
    let mut title = String::new();
    let mut description = String::new();
    let mut gross = String::new();

    println!("\nEnter title: ");
    stdin().read_line(&mut title).unwrap();

    println!("\nEnter description: ");
    stdin().read_line(&mut description).unwrap();

    loop {
        println!("\nEnter gross amount eg 100: ");
        stdin().read_line(&mut gross).unwrap();

        if gross.trim().parse::<f64>().is_ok() {
            break;
        }
    }

    Breakdown::new(title, description, gross.trim().parse::<f64>().unwrap())
}

fn collect_expense() -> Expense {
    let mut title = String::new();
    let mut description = String::new();
    let mut cost = String::new();

    println!("\nEnter title: ");
    stdin().read_line(&mut title).unwrap();

    println!("\nEnter description: ");
    stdin().read_line(&mut description).unwrap();

    let parsed_cost = loop {
        println!("\nEnter cost eg 100: ");
        stdin().read_line(&mut cost).unwrap();

        let parsed_cost = cost.trim().parse::<f64>();

        if parsed_cost.is_ok() {
            break parsed_cost.unwrap();
        }

        println!("Invalid input ==> {cost}");
    };

    Expense::new(title, description, parsed_cost)
}

fn start_breakdown_operation(breakdown: &mut Breakdown) {
    loop {
        println!("\nEnter action: ");
        println!("0 - Add expense");
        println!("1 - Show breakdown");
        println!("Type 'exit' to exit.");

        let mut breakdown_action = String::new();
        stdin().read_line(&mut breakdown_action).unwrap();

        if breakdown_action.trim() == "exit" {
            break;
        }

        match breakdown_action.trim().parse::<u8>() {
            Ok(0) => breakdown.add_expense(collect_expense()),
            Ok(1) => breakdown.show_breakdown(),
            _ => println!("\nInvalid input."),
        }
    }

    println!("\nThanks for using Breakdown Calculator!");
    println!("Bye!");
}

fn main() {
    println!("Welcome to Breakdown Calculator!");
    println!("It calculates gross minus expenses and gives you breakdown with balance.");

    let mut breakdown = initialize_breakdown();

    start_breakdown_operation(&mut breakdown);
}
