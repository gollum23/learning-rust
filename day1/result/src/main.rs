use std::collections::HashMap;
use std::env::args;

fn main() {
    let mut hm = HashMap::new();

    hm.insert(3, "Hello");
    hm.insert(5, "world");

    let r = match hm.get(&3) {
        Some(v) => v,
        _ => "NOTHING"
    };

    println!("{}", r);

    let q = hm.get(&4).unwrap_or(&"NOTHING");

    println!("{}", q);

    match "3".parse::<f32>() {
        Ok(v) => println!("Ok - {}", v),
        Err(e) => println!("Error - {}", e)
    }

    match "3t".parse::<f32>() {
        Ok(v) => println!("Ok - {}", v),
        Err(e) => println!("Error - {}", e)
    }

    match get_arg(3) {
        Ok(v) => println!("Ok - {}", v),
        Err(e) => println!("Error - {}", e)
    }
}

fn get_arg(n:usize) -> Result<String, String> {
    for (i, a) in args().enumerate() {
        if i == n {
            return Ok(a)
        }
    }
    Err("Not enough Args".to_string())
}
