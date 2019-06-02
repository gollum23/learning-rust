use std::ops::Add;

pub struct Point {
    x:i32,
    y:i32,
}

// Implement add method for point struct
impl Add for Point {
    // Define output type
    type Output = Point;
    // Override method add
    fn add(self, other:Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

fn main() {
    let a = Point{x:3, y:5};
    let b = Point{x:30, y:50};

    let c = a + b;

    println!("c = x:{}, y:{}", c.x, c.y);
}
