fn main() {
    println!("strings");

    //
    let s = String::new();
    println!("s - {}", s);
    let data = "initial contents";
    let s = data.to_string();
    println!("s - {}", s);
    let s2 = "initial contents".to_string();
    println!("s2 - {}", s2);
    let s3 = String::from("initial contents");
    println!("s3 - {}", s3);

    //
    let mut s1 = String::from("foo");
    s1.push_str("bar");
    let s2 = "zoo";
    s1.push_str(s2);
    s1.push('.');
    println!("s1 is {s1}; s2 is {s2}");

    //
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s1 + s2 = {s3}");

    //
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s1 + s2 + s3 = {s}");
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("format s1-s2-s3 - {s}");

    //
    let hello = "Здравствуйте";
    let answer = &hello[0..4];
    println!("{answer}");

    //
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
