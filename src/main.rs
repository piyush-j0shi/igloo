use std::f64::consts::PI;

trait Draw {
    fn draw(&self) -> String;
}

trait Area {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Square {
    parametre: f64,
}

impl Draw for Square {
    fn draw(&self) -> String {
        format!(
            "{}",
            r#"+----+
|    |
+----+"#
        )
    }
}

impl Draw for Circle {
    fn draw(&self) -> String {
        format!(
            "{}",
            r#" /\
/  \
\  /
 \/"#
        )
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.parametre * self.parametre
    }
}

fn lets_see<T>(value: T)
where
    T: Draw + Area,
{
    let ascii_drawing = value.draw();
    let area_calculate = value.area();

    println!("ascii drawing is ");
    println!("{ascii_drawing}");
    println!("area is : {area_calculate}");
}

fn main() {
    let circle = Circle { radius: 10.0 };
    lets_see(circle);

    let square = Square { parametre: 10.0 };
    lets_see(square);
}
