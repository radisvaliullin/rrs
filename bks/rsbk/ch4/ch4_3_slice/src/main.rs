fn main() {
    println!("slices");

    // find string part
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    println!("word - {}", word);

    // or use slice
    let s2 = String::from("hello world");
    let hello = &s2[0..5];
    let world = &s2[6..11];
    println!("h - {}, w - {}", hello, world);

    //
    let s3 = String::from("qwerty asdf zxcv");
    let first = first_word_slice(&s3);
    // commented, if we try change s3 (neet set s3 mutable also) will get panic because ref passed to func is imutable and first expected imutable
    // s3.clear(); 
    println!("first - {}", first);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
