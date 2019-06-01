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

    // Reserved word continue
    let m = vec![4, 7, 8, 9, 10, 11];

    for n in m {
        if n % 2 == 0 {
            continue;
        }
        println!("Continue {}", n)
    }

    // Reserved word break
    let a = vec![4, 7, 8, 9, 11, 10];

    for n in a {
        if n == 11 {
            break;
        }
        println!("Break {}", n)
    }

    // Nested loop
    // https://doc.rust-lang.org/rust-by-example/flow_control/loop/nested.html
    let c = vec![4, 7, 8, 9, 11, 10];

    'outer: for i in 0..10 {
        for n in &c {
            if i + *n == 11 {
                break 'outer;
            }
            println!("nested {}", n)
        }
    }
}