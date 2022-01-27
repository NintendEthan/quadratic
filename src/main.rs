
mod help;
mod quadratic_formula;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let function = &args[1];
    match function.as_str() {
        "help" => help::help(),
        "formula" => {
            let arg_a = match args[2].trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Error parsing output.");
                    std::process::exit(1);
                }
            };
            if arg_a == 0 {
                println!("'a' must be non-zero!");
                std::process::exit(1);
            }
            let arg_b = match args[3].trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Error parsing output.");
                    std::process::exit(1);
                }
            };
            let arg_c = match args[4].trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Error parsing output.");
                    std::process::exit(1);
                }
            };
            quadratic_formula::quadratic_formula(arg_a, arg_b, arg_c);
        }
        _ => {
            println!("Please enter valid function! Invalid funtion {}", function);
            std::process::exit(1);
        }
    
    }
}
