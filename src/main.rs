#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn swap(self) -> Point<U, T> {
        Point {
            x: self.y,
            y: self.x,
        }
    }
}

fn main() {
    let point = Point {
        x: "x".to_string(),
        y: 32,
    };
    println!("before swap : {:?}", point);

    let swappoed_point = point.swap();
    println!("after swap : {:?}", swappoed_point);
}
