fn main() {
    
    // Scalar

    // float64
    let n: f64 = "42.2".parse().expect("not a number");
    println!("num: {n}");

    // int type
    // mut uses for overflow case (see next section)
    // let mut i: u8 = 255;
    let i: u8 = 255;
    println!("i before overflow: {i}");

    // int overflow
    // commented, in debug mode fails
    // i = i+1;
    // println!("i overflow: {i}");

    // boolean
    let t = true;
    let f: bool = false;
    println!("boolean: t - {t}; f - {f}");

    // char
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("chars: ascii - {c}, unicode - {z}, emoji - {heart_eyed_cat}");

    // Compound

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (i, f, u) = tup;
    println!("tuple - {i}, {f}, {u}");

    // tuple indexing
    let tup2: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup2.0;
    let six_point_four = tup2.1;
    let one = tup2.2;
    println!("tup2 - {five_hundred}, {six_point_four}, {one}");

    // array
    let a = [1, 2, 3, 4, 5];
    println!("arr - {:?}", a);

    // array string
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    println!("str arr - {:?}", months);

    //
    let a = [3; 5];
    println!("arr2 - {:?}", a);
    // will raise panic on compile time because array out of index
    // let aelem = a[73];
    // println!("arr elem - {:?}", aelem);
}
