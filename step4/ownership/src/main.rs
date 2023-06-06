/**
Ownership Rules
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.
*/
fn main() {
    ownership();
    reference_borrowing();
    slice();
}

fn ownership() {
    {
        let s = "hello";
    } // At the end of this block, variable `s` is invalid.

    let x = String::from("hello");
    let y = x; // ownership moved `x` to `y`
    let z = pass_argument(y); // even function invocation causes ownership movement
    // println!("x = {}", x); // so this causes error
    let s = generate();

    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy

    let x = 1;
    let y = x; // variable `x` is still valid, because `x` stored in 'stack memory'

    fn pass_argument(s: String) {
        // ...
    }

    fn generate() -> String {
        return String::from("hello"); // `return` also causes ownership movement
    }
}

fn reference_borrowing() {
    fn length(s: &String) -> usize {
        return s.len();
    }

    fn append(s: &mut String) {
        s.push_str(" world");
    }

    let s1 = String::from("hello");
    let len = length(&s1);
    println!("s = {}", s1); // This is still valid, because ownership borrowed, not moved.

    let mut s2 = String::from("hello");
    append(&mut s2); // Note that `mut` required when invocation
    let r1 = &mut s2;
    // let r2 = &mut s2; // this can cause error because ...
    // println!("r1 = {}, r2 = {}", r1, r2); // only one variable can reference mutable variable in same scope

    let r3 = &s2;
    let r4 = &s2; // This is OK.
    // let r5 = &mut s2; // But this is also not allowed, because immutable reference exists(`r3`, or `r4`)

    fn dangling_reference() {
        // Not allowed.
        /*
        fn dangle() -> &String {
            let s = String::from("hello");
            &s
        }
         */

        // Return directly instead.
        fn no_dangle() -> String {
            let s = String::from("hello");
            s
        }
    }
}

fn slice() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    let hello_world = &s[0..s.len()];
    let hello_world = &s[..];

    fn first_word(s: &str/* better than &String */) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    let string_from = String::from("from");
    let string_literal = "literal";

    println!("{}", first_word(&string_from));
    println!("{}", first_word(&string_from[..]));
    println!("{}", first_word(string_literal));
    println!("{}", first_word(&string_literal));
    println!("{}", first_word(&string_literal[..]));
    // str > String

    let arr = [1, 2, 3];
    let x = &arr[1..2];


}


