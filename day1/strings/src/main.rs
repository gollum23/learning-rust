fn main() {

    let s = String::from("Hello 汉字");

    for c in s.chars() {
        println!("{}", c);
    }

    for c in s.bytes() {
        println!("{}", c);
    }

    let m = "Hello 汉字";

    for c in m.chars() {
        println!("{}", c);
    }

    for c in m.bytes() {
        println!("{}", c);
    }

    // Get the len in bytes
    println!("S Len = {}", s.len());

    // Use enumerate to get indices
    for (i, c) in s.chars().enumerate() {
        println!("Enumerate {} = {}", i, c)
    }

    // use char_indices method instead of enumerate
    for (i, c) in s.char_indices() {
        println!("Indices {} = {}", i, c)
    }

    println!("S Len = {}", count_l(m));
    println!("S Len = {}", count_l(&s));
}

fn count_l(s:&str) -> i32 {
    let mut res = 0;
    for c in s.chars() {
        if c == 'l' {
            res += 1
        }
    }
    res
}
