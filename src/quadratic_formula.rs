struct QuadVars {
    a: i32,
    b: i32,
    c: i32
}

pub fn quadratic_formula(a: i32, b: i32, c: i32) {
    let eq = build_eq(a, b, c);
    let result = quad_form(eq);
    let result_plus = result.0;
    let result_minus = result.1;
    println!("\n{0} or {1}",result_plus, result_minus);
}

fn build_eq(a: i32, b: i32, c: i32) -> QuadVars {
    QuadVars {
        a,
        b,
        c
    }
}

fn quad_form(quad_vars: QuadVars) -> (f64, f64) {
    let a: f64 = quad_vars.a as f64;
    let b: f64 = quad_vars.b as f64;
    let c: f64 = quad_vars.c as f64;
    let negative_b: f64 = b * -1.0;
    let pre_squared: f64 = (b * b) - 4.0 * (a * c);
    let squared = pre_squared.sqrt();
    if squared.is_nan() == true {
        println!("Tried to take square root of a negative number. Check your equation!");
        std::process::exit(126);
    }
    let top_plus = negative_b + squared;
    let top_minus = negative_b - squared;
    let final_plus = top_plus / (a * 2.0);
    let final_minus = top_minus / (a * 2.0);
    let result = (final_plus, final_minus);
    return result;
}