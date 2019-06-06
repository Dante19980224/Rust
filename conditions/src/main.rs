fn main() {
    let n = 2;

    if n < 5 {  // parentheses are UNNECESSARY
        println!("n < 5");
    } else if n == 5 {
        println!("n == 5");
    } else {
        println!("n > 5");
    }

    let b = true;
    let n2 = if b { // types must match
        1
    } else {    // types must match
        0
    };
    println!("n2 is {}", n2);

    //  loops
    let mut n3 = 0;

    loop {
        println!("n3 is {}", n3);
        n3+=1;  // ++ is not a thing in Rust!! Only +=
        if n3 == 5 {
            break;
        }
    }

    // label loops
    let mut n4 = 0;
    'la: loop{  // need that ' in front of label
        println!("loop a");
        'lb: loop{
            println!("loop b");
            'lc: loop{
                println!("loop c");
                n4 += 1;
                if n4 <= 4 {
                    continue 'lc    // go to the beginning of a specific loop
                } else {
                    break 'lb   // break a specific loop
                }
            }
        }
        break // break current loop
    }

    // while loops
    let mut n5 = 0;

    while n5 < 3 {
        println!("n5 is {}", n5);
        n5+=1;
    }

    // for loops
    let vec1 = vec![1,2,3,4,5];
    for i in vec1{
        print!("{} ", i);
    }
    println!("");

    for i in 0..10 { // 10 times    0..10 is exclusive(10 times)      0..=10 is inclusive(11 times)
        print!("{} ", i);
    }
    println!("");

    // match
    let n6  = 420;

    match n6 {
        1 => println!("n6 is 1"),
        2 => println!("n6 is 2"),
        3 | 4 | 5 => println!("3 <= n6 <= 5"),
        1..=10 => println!("n6 <= 10"),
        1..=420 => println!("n6 <= 420"),   // cannot use exclusive pattern here, yet?(experimental)
        _ => println!("n6 is weird"),
    }

    let p = (5, 23 , 806);

    match p {
        (10, _, x) => println!("{}", x),
        (x, y, _) if (x-2)*y == 69 => println!("yes"),
        _ => println!("w/e"),
    }

    let n7 = match n6 {
        n7 @ 1..=200 => n7,
        n7 @ 201..=400 => n7,
        n7 @ 401..=420 => n7,
        _ => 420,
    };
    println!("n7 is {}", n7);
}
