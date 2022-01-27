mod help;
mod quadratic_formula;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("Please input a valid function ( help for help )");
        std::process::exit(1);
    }
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
#[cfg(test)]
mod tests {
    #[test]
    fn sanity() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_quad_form() {
        use crate::quadratic_formula::QuadVars;
        let test_eq = QuadVars { a: 1, b: -2, c: -15 };
        let result = test_eq.quad_form();
        assert!(result == (5.0, -3.0));
    }
}


