fn main() {
    let s = "hello".to_string();
    myprint(&s);
    myprint(&s);
}
fn myprint<T: std::fmt::Display>(msg: &T){
    println!("{}", msg)
}
