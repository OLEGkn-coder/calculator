use clap::{Parser, Subcommand};
use std::fs;
#[derive(Parser)]
#[command(
    author = "Oleh Moshenskyi",
    version = "1.0",
    about = "Parser-calculator for parsing a mathematical expression",
    long_about = "In the parser, we will parse a \
    mathematical expression into its main components: \
    numbers, signs, brackets. The result of the parsing \
    will be used to calculate the mathematical expression, \
    taking into account priorities: first the brackets, \
    then multiplication/dividing and at the end addition \
    and subtraction. The calculator also takes into account \
    the nested brackets. The calculator also raises the number \
    to the power (pow(n, p)), calculates the square root \
    (sqrt(n)) and calculates sin(n) and cos(n)."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ///Parse you file and calculate the result
    Parse { file: String },
    ///Show the authors
    Authors,
    ///Show the version
    Version,
    ///Show about
    About,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Parse { file } => {
            println!("Welcome to the calculator");
            println!("y-y-a-a");
            println!("Parsing file: {file}");
            let contents = fs::read_to_string(file).expect("Error reading file");
            let res = Parser_calculator::check_bracket(&contents);
            println!("File:\n{file}");
            println!("Result: {}", res.unwrap());
        }
        Commands::Authors => println!("Oleh Moshenskyi"),
        Commands::Version => println!("Parser-calculator v1.0"),
        Commands::About => println!(
            "In the parser, we will parse a \
        mathematical expression into its main components: \
        numbers, signs, brackets. The result of the parsing \
        will be used to calculate the mathematical expression, \
        taking into account priorities: first the brackets, \
        then multiplication/dividing and at the end addition \
        and subtraction. The calculator also takes into account \
        the nested brackets. The calculator also raises the number \
        to the power (pow(n, p)), calculates the square root \
        (sqrt(n)) and calculates sin(n) and cos(n)."
        ),
    }
}
