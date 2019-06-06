#![allow(dead_code)]    // suppress warning about unused code(unused enums)

enum MyInfo {
    Fullname{firstname: String, lastname: String},
    Age(i32),
    Address(String, String, String),
    Yes,
}

#[derive(Debug)]
enum Directions{
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
enum Keys{
    UpKey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String),
}

impl Directions {
    fn match_direction(&self) -> Keys {
        match *self {
            Directions::Up(_) => Keys::UpKey(String::from("Pressed w")),
            Directions::Down(_) => Keys::DownKey(String::from("Pressed s")),
            Directions::Left(_) => Keys::LeftKey(String::from("Pressed a")),
            Directions::Right(_) => Keys::RightKey(String::from("Pressed d")),
        }
    }
}

impl Keys {
    fn destruct(&self) -> &String{
        match *self{
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}

enum Shapes{
    Rectangle{length: u32, width: u32},
    Square(i64),
}

impl Shapes{
    fn area(&self) -> i64{
        match *self{
            Shapes::Rectangle{length, width} => (length * width) as i64, // casting in rust
            Shapes::Square(ref l) => l * l,
        }
    }
}

// option division
fn division(a: f64, b: f64) -> Option<f64>{
    if b == 0.0 {
        None
    } else {
        Some(a/b) 
    }
}

fn main() {
    // let myname = MyInfo::Fullname{firstname: "Harry".to_string(), lastname: "Balls".to_string()};
    // let myage = MyInfo::Age(12);
    // let mywhat = MyInfo::Yes;
    let u = Directions::Up(Point{x : 0.12, y: 1.23});
    let k = u.match_direction();
    let l = k.destruct();

    println!("{:?}", u);
    println!("{:?}", k);
    println!("{:?}", l);

    // to cast in Rust, use keyword "as"
    let a : u32 = 10;
    let b : i64 = a as i64;
    println!("{}", b);

    let r1 = Shapes::Rectangle{length: 3, width: 4};
    let s1 = Shapes::Square(5);
    println!("area of r1 is {}", r1.area());
    println!("area of s1 is {}", s1.area());
    
    let div1 = division(6.66, 0.0);
    let div2 = division(52314.23123, 341.2);
    match div1 {
        Some(x) => println!("{:.4}", x),   // 4 digits after decimal
        _ => println!("0 division"),
    }
    match div2 {
        Some(x) => println!("{:.4}", x),   // 4 digits after decimal
        _ => println!("0 division"),
    }
}
