fn main() {
    loop_to_10();
    println!("Hello, world!");
    loop_10();
    while_to_10();
    array_loop();
}


fn loop_10() {
    let mut n = 0;
    loop {
        n += 1;
        println!("hello");
        if n >= 10 {
            return;
        }
    }
}

fn loop_to_10() {
    for n in 0..10 {
        println!("Hello {}", n)
    }
}

fn while_to_10() {
    let mut n = 0;
    while n < 10 {
        println!("Hello while {}", n);
        n += 1;
    }
}

fn array_loop() {
    let mut v = Vec::new();
    v.push(4);
    v.push(7);
    v.push(9);

    for n in v {
        println!("{}", n)
    }

    let b = vec![3, 6, 8];

    for n in b {
        println!("{}", n)
    }

}