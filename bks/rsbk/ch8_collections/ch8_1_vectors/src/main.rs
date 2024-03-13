fn main() {
    println!("vecotrs");

    let v: Vec<i32> = Vec::new();
    println!("v vector - {:?}", v);

    let v2 = vec![1, 2, 3];
    println!("v2 - {:?}", v2);

    //
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    println!("v3 - {:?}", v3);

    //
    let v4 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v4[2];
    println!("The third element is {third}");
    let third: Option<&i32> = v4.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // commented, raise panic because we can push having first reference
    // let mut v5 = vec![1, 2, 3, 4, 5];
    // let first = &v5[0];
    // v5.push(6);
    // println!("The first element is: {first}");

    //
    let mut v6 = vec![100, 32, 57];
    println!("v6 - {:?}", v6);
    for i in &mut v6 {
        *i += 50;
    }
    println!("v6 - {:?}", v6);

    //
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row - {:#?}", row);
}
