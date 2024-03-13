use add_one;

fn main() {
    println!("test cargo workspace");

    let num = 10;
    println!("{num} plus one is {}!", add_one::add_one(num));
}
