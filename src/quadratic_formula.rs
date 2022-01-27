struct QuadVars {
    a: i32,
    b: i32,
    c: i32
}

impl QuadVars {
    fn quad_form(&self) -> (f64, f64) {
        let negative_b: f64 = self.b as f64 * -1.0;
        let pre_squared: f64 = (self.b as f64 * self.b as f64) - 4.0 * (self.a as f64 * self.c as f64);
        let squared = pre_squared.sqrt();
        if squared.is_nan() == true {
            println!("Tried to take square root of a negative number. Check your equation!");
            std::process::exit(126);
        }
        let top_plus = negative_b + squared;
        let top_minus = negative_b - squared;
        let final_plus = top_plus / (self.a as f64 * 2.0);
        let final_minus = top_minus / (self.a as f64 * 2.0);
        let result = (final_plus, final_minus);
        return result;
    }
}

pub fn quadratic_formula(a: i32, b: i32, c: i32) {
    let eq = QuadVars {a, b, c};
    let result = eq.quad_form();
    let result_plus = result.0;
    let result_minus = result.1;
    println!("\n{0} or {1}",result_plus, result_minus);
}
