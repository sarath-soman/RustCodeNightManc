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
    match args.operation {
        '+' => {
            let result = args.num1 + args.num2;
            println!("The result is {}", result);
        }
        '-' => {
            let result = args.num1 - args.num2;
            println!("The result is {}", result);
        }
        'x' | 'X' => {
            let result = args.num1 * args.num2;
            println!("The result is {}", result);
        }
        '/' => {
            let result = args.num1 / args.num2;
            println!("The result is {}", result);
        }
        _ => unimplemented!()
    }    
}