#[derive(Debug)]
struct One {
    _one: u32,
    _two: String,
}

pub fn main() {
    println!("mainmain");

    let a = 1;
    let b = a;
    println!("a is {}", a);
    println!("b is {}", b);

    let s = String::from("asdf");
    let s2 = s;
    // comment because s moved to s2
    // println!("s is {}", s);
    println!("s2 is {}", s2);

    let ss = "zxcv";
    let ss2 = ss;
    println!("ss is {}", ss);
    println!("ss2 is {}", ss2);

    let o = One{_one: 1, _two: String::from("qwerty")};
    let o2 = o;
    // comment because o moved to o2
    // println!("o is {:?}", o);
    println!("b is {:?}", o2);
}
