use std::f64::consts::PI;

trait Area {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    width: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }
}

fn main() {
    let cicrle = Circle { radius: 10.0 };
    let rectangle = Rectangle {
        length: 10.0,
        width: 10.0,
    };

    let area_circle = cicrle.area();
    let rectangle_area = rectangle.area();

    println!("area of circle : {}", area_circle);
    println!("area of rectangle : {}", rectangle_area);
}
