
/*
Create a public struct
pub struct {

}
*/
#[derive(Debug)]
struct User{
    name:String,
    age:i32,
    height:i32,
    shoesize:i32,
}

impl User {
    fn simple_string(&self) -> String {
        return format!("{} - {} - {} cm - shoe: {}", self.name, self.age, self.height, self.shoesize)
    }

    fn grow(&mut self, h:i32) {
        return self.height += h;
    }

    fn die(self) {
        return println!("Dead {}", self.simple_string());
    }
}

fn main() {

    let mut u = User {
        name: "Gollum".to_string(),
        age: 25,
        height: 172,
        shoesize: 39
    };

    println!("User is {:?}", u);
    println!("User is {}", u.simple_string());
    u.grow(20);
    println!("User is {}", u.simple_string());
    u.grow(10);
    u.die();
}
