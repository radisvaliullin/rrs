fn main() {
    println!("ownership");

    let s = "hello";
    println!("s - {}", s);

    let mut ss = String::from("hello");
    ss.push_str(", world!");
    println!("ss - {}", ss);

    // commented, code will raise panic
    // raise panic because s1 was moved to s2
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("s1 - {}, world!", s1);

    // string clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    //
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // ownership and function
    let s3 = String::from("hello");
    takes_ownership(s3);
    let x3 = 5;
    makes_copy(x3);
    // commented, eraise panic because ownership passed to function
    // println!("s3 = {}, x3 = {}", s3, x3);

    // return value and scope
    let s4 = gives_ownership();
    let s5 = String::from("hello");
    let s6 = takes_and_gives_back(s5);
    println!("s4 - {}, s6 - {}", s4, s6);

    // use and return back
    let s7 = String::from("hello");
    let (s8, len) = calculate_length(s7);
    println!("length of '{}' is {}.", s8, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
