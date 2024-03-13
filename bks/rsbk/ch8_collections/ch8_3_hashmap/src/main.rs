fn main() {
    println!("hashmap");

    use std::collections::HashMap;
    let mut kv = HashMap::new();

    kv.insert(String::from("one"), 1);
    kv.insert(String::from("two"), 2);
    println!("kv - {:?}", kv);

    //
    let v = kv.get("two").copied().unwrap_or(0);
    println!("v - {}", v);

    //
    for (k, v) in &kv {
        println!("key - {k}: value - {v}");
    }

    //
    let k = String::from("one");
    let v = String::from("oneone");

    let mut map = HashMap::new();
    map.insert(k, v);
    println!("map - {:?}", map);
    // commented, raise panic becuase value v is now owned by map;
    // println!("value - {v}");

    //
    let mut kv2 = HashMap::new();
    kv2.insert(String::from("one"), 10);
    kv2.insert(String::from("one"), 25);
    println!("map kv2 - {:?}", kv2);
    //
    kv2.insert(String::from("two"), 10);
    kv2.entry(String::from("three")).or_insert(50);
    kv2.entry(String::from("two")).or_insert(50);
    println!("map kv2 - {:?}", kv2);

    //
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map (counter of word) - {:?}", map);
}
