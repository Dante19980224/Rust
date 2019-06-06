use std::fmt;

#[derive(Debug)]    // enable debug println
struct Object {
    width: u32,
    length: u32,
}

// constuctor
impl Object {
    fn new(width: u32, length: u32) -> Object {
        Object {
            width: width,
            length: length,
            // Since the argument names are the same as data field names, we can use the following 2 lines instead
            // width,
            // length,
        }
    }
}

// same as doing it in 1 block, no side effects
// methods
impl Object {
    fn area(&self) -> u32 {
        self.length*self.width
    }

    fn show(&self){
        println!{"(impl show fn)Object obj has width: {}, lenght: {}, and area: {}", self.width, self.length, self.area()};
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { // fmt::Formatter is mainly used to write into files and console
        write!(f, "({}, {})", self.width, self.length)
    }
}

fn area2(obj: &Object) -> u32 {
    obj.length * obj.width
}

fn main() {
    println!("****This here is main****");
    let o = Object {
        width: 3,
        length: 4,
    };

    println!{"(outside fn)Object o has width: {}, lenght: {}, and area: {}", o.width, o.length, area2(&o)};

    let obj = Object::new(10, 20);
    obj.show();

    println!("{:?}", o); // add derive to struct to enable this debug println
    println!("{:#?}", obj); // pretty debug
}
