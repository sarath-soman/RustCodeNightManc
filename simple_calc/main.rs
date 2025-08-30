use clap::Parser;

/// Simple program to add 2 numbers
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    /// number 1
    num1: f64,

    /// Operator
    operation: char,

    /// Number 2
    num2: f64,
}

fn main() {
    let args = Args::parse();
    let result = run_calculation(args.num1, args.operation, args.num2);
    println!("The result is {:?}", result);
}

fn run_calculation(num1: f64, operation: char, num2: f64) -> f64 {
    match operation {
        '+' => num1 + num2,
        '-' => num1 - num2,
        'x' | 'X' => num1 * num2,
        '/' => num1 / num2,
        _ => unimplemented!()
    }
}