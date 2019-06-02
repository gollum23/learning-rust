#[derive(Debug)]
pub struct Bed {
    size:i32,
    count:u32,
}

#[derive(Debug)]
pub enum Room {
    Kitchen(i32),
    Bedroom(Bed),
    Lounge,
}

fn main() {
    use self::Room::*;
    let t = Bedroom(Bed{size:190, count:2});
    println!("Hello from the {:?}!", t);

    let v = match t {
        Kitchen(n) => n, _d => 0
    } + 10;

    println!("v = {}", v)
}
