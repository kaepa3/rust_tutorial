struct RightTriangle {
    base: f64,
    perpendicular: f64,
}
impl RightTriangle {
    fn area(&self) -> f64 {
        (self.base * self.perpendicular) * 0.5
    }
    fn length(&self) -> f64 {
        self.base + self.perpendicular + (self.base.powi(2) + self.perpendicular.powi(2)).sqrt()
    }
}
struct Rectangle<T,S> {
    width: T,
    height: S,
}

impl<T,S> Rectangle<T,S> {
    fn new(width: T, height:S)->Rectangle<T,S>{
        Rectangle{
            width: width,
            height:height,
        }
    }
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn length(&self) -> f64 {
        (self.width + self.height) * 2.0
    }
}
fn main() {
    let tri = RightTriangle {
        base: 3.0,
        perpendicular: 4.0,
    };
    println!("{}", tri.area());
    println!("{}", tri.length());

    let rec = Rectangle::<i32,f64>{
        width: 2,
        height: 4.0,
    };
    println!("{}", rec.area());
    println!("{}", rec.length());
}
