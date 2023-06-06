const CONST_VAL: u32 = 100;

fn main() {
    mutability();
    data_types();
    function();
    control_statements()
}

fn mutability() {
    // let x = 5; // This is impossible
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("Constant value = {}", CONST_VAL);

    // shadowing
    let y = 1;
    let y = 2;
    let y = 3;
    println!("y = {}", y);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces = {}", spaces);
}

fn data_types() {
    // data types
    let number: u32 = "42".parse().expect("Not a number!"); // Type must be specified.

    // integer types
    /*
    i8, i16, i32, i64
    u8, u16, u32, u64
    * Note that i32 faster even with 64bit system
    */
    // let integer: i8 = 99999; // out of range
    // let integer: u32 = -1; // out of range
    let integer: isize = 32; // determined by architecture

    // integer literal
    let decimal: i32 = 12_34_5;
    let hex: i32 = 0xff;
    let octal: i32 = 0o77;
    let binary: i32 = 0b111_000;
    let byte: u8 = b'A'; // u8 only

    // float
    /*
    f64 is recommended than f32
     */

    // tuple
    let tup = (123, 4.5, "hello");
    let (x, y, z) = tup; // destruction;
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    // let a = tup.3; // out of range;

    // array
    let arr = [1, 2, 3, 4, 5]; // must be all same types
    // let x = arr.1; // impossible
    let x = arr[0]; // use indexing this instead
    // let y = arr[1..2]; // but can't use slicing
    // let e = arr[10]; // this can be compiled, but error on runtime
}

fn function() {
    fn returnfun() -> i32 {
        1 // same with `return 1;` because rust allows expression
    }

    fn argument(a: i32, b: i32) {
        println!("a = {}", a);
        println!("b = {}", b);
    }
    
    argument(1, 2);
    returnfun();
}

fn control_statements() {
    let number = 1;
    let other = if number == 1 { 1 } else { 2 }; // type must be same.
    // if 1 { println!("hello") } // only boolean allowed

    loop { // infinite loop
        break;
    }

    while number == 2 { println!("hello") } // conditional loop

    for number in (1..4).rev() { // start inclusive, end exclusive
        println!("iteration number = {}", number);
    }
}
