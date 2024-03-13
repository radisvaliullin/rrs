fn main() {

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // shadow
    let x2 = 5;
    let x2 = x2 + 1;
    {
        let x2 = x2 * 2;
        println!("The value of x2 in the inner scope is: {x2}");
    }
    println!("The value of x2 is: {x2}");
}
