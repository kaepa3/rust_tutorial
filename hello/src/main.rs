fn main() {
    for i in 1..=10 {
        println!("Hello, {} world!", to_oridinal_string(i));
    }
}
fn to_oridinal_string(n: usize) -> String {
    let s = match n % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };
    format!("{}{}", n, s)
}
