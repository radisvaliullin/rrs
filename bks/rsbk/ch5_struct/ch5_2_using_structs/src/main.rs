#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("using structure");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "the area of the rectangle\n{:#?}\nis {} square pixels.",
        rect1, area(&rect1)
    );
    // or
    println!("rect1 dbg is:");
    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
