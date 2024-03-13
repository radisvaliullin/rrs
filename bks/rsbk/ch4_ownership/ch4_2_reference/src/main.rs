fn main() {
    println!("references");

    // reference imutable
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("length of '{}' is {}.", s1, len);

    // mutable
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("s2 is {}.", s2);

    // commented, raise panic because r2 try use s3 already referenced by r1
    // let mut s3 = String::from("hello");
    // let r1 = &mut s3;
    // let r2 = &mut s3;
    // println!("{}, {}", r1, r2);

    // it is ok because reference in different scopes
    let mut s4 = String::from("hello");
    {
        let _r3 = &mut s4;
    }
    let r4 = &mut s4;
    println!("r4 - {}", r4);

    // commented, raise panic, we can not use mutable reference if we already have immutable
    // but we can have multiple immutable references
    // let mut s5 = String::from("hello");
    // let r5 = &s5; // no problem
    // let r6 = &s5; // no problem
    // let r7 = &mut s5; // BIG PROBLEM
    // println!("{}, {}, and {}", r5, r6, r7);

    // ok because after print r8 and r9 not more used we do not have more reference to s6
    let mut s6 = String::from("hello");
    let r8 = &s6;
    let r9 = &s6;
    println!("{} and {}", r8, r9);
    let r10 = &mut s6;
    println!("r10 - {}", r10);

    // danglin reference
    // commented, raise panic becaue returned not used value
    // let reference_to_nothing = dangle();
    // println!("ref to nothing - {}", reference_to_nothing);

    // ok
    let no_d = no_dangle();
    println!("no dangle - {}", no_d);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
