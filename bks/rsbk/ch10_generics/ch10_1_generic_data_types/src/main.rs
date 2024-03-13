struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    println!("generic data types");

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point {
        x: "qwerty",
        y: 'c',
    };
    let p3 = p1.mixup(p2);
    println!("p3 fields: p3.x = {}, p3.y = {}", p3.x, p3.y);

    //
    let p4 = Point { x: 2.0, y: 4.0 };
    let res = p4.distance_from_origin();
    println!("p4 - {}", res);
}
