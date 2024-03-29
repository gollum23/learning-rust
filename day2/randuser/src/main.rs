extern crate rand;

use rand::Rng;

use std::ops::Add;

pub struct Point {
    x:i32,
    y:i32,
}

impl Point {
    fn random() -> Self{
        let mut tr = rand::thread_rng();
        Point {
            x: tr.gen(),
            y: tr.gen(),
        }
    }
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

    let d = Point::random();

    println!("c = x:{}, y:{}", c.x, c.y);
    println!("d = x:{}, y:{}", d.x, d.y);
}
