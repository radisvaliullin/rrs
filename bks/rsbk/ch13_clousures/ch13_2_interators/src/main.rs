#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn main() {
    //
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    println!("iterate vector: {:?}", v1);
    for val in v1_iter {
        println!("next item: {}", val);
    }

    //
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    // commented other examles into an mut. into - owned v1, mut - return mut ref
    // let mut v1_iter = v1.into_iter();
    // let mut v1_iter = v1.iter_mut();
    println!("iterate vecotr: {:?}", v1);
    println!("next {:?}", v1_iter.next());
    println!("next {:?}", v1_iter.next());
    println!("next {:?}", v1_iter.next());
    println!("next {:?}", v1_iter.next());

    //
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("sum: {}", total);

    //
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v1 - {:?}; v2 - {:?}", v1, v2);

    //
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    println!("shoes: {:?}", shoes);
    let my_size = 10;
    let in_my_size = shoes_in_size(shoes, my_size);
    println!("my size - {}, shoes_in_my_size - {:?}", my_size, in_my_size);
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
