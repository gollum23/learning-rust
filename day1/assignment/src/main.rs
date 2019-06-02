use std::env::args;

fn main() {
    // My Solution
    match get_arg() {
        Ok(v)  => println!("Hello {}", v),
        Err(_e) => {}
    }

    // Course Solution
    for a in args() {
        if let Some(c) = a.chars().next() {
            match c {
                'w' | 'W' => println!("Hello {}", a),
                _ => {}
            }
        }
    }

    for a in args() {
        if a.starts_with(&"w") | a.starts_with(&"W") {
            println!("Hello {}", a)
        }
    }
}

// My Solution
fn get_arg() -> Result<String, String> {
    for a in args() {
        if a.starts_with(&"W") {
            return Ok(a)
        }
    }
    Err("".to_string())
}
