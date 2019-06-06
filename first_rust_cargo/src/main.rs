fn main() {
    println!("Hello, world!");
    
    let x = 5;
    // let mut x2 = 5;
    // x2 = 10;
    // let x_unsigned32:u32 = 20/x2;

    // let b:bool = true;
    
    // let c:char = 'c';

    let tuple:(isize, f64, char) = (420, 4.20, 't');
    let (x, _, _) = tuple;
    
    // let a:[i32; 5] = [1,2,3,4,5];
    // let a_0 = a[0];
    // let a_len = a.len();
    // let a_234 = &a[1..4];

    println!("{:?}", tuple);
    let s = String::from("tuple pattern matching variable name can be the same as a variable name defined earlier. When called, the previously defined variable would be consider as an unused variable");
    println!("{}\n{}",s, x);
    let s_tuple = &s[0..5];
    println!("{}", s_tuple);
    let s_pre = String::from("Message: ");
    let s_full = s_pre + &s;
    println!("{}", s_full);

    // let unit_val = (); // void functions(such as main) return empty tuple(unit)

    {
        // this is a scope
        let a = 1;
    }
    // cant access a from outside of scope

    

}
