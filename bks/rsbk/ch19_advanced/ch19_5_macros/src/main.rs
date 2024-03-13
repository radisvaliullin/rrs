use bks_rsbk_ch19_5::myvec;
use bks_rsbk_ch19_5::HelloMacro;
use bks_rsbk_ch19_5_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    let v = myvec![1, 2, 4, 8];
    println!("myvec - {:?}", v);

    Pancakes::hello_macro();
}
