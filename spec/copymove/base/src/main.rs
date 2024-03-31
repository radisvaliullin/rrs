#[derive(Debug)]
struct One {
    _one: u32,
    _two: String,
}

#[derive(Debug)]
struct OneOne<'a> {
    _one: u32,
    _oneone: &'a mut Box<One>,
}

// impl<'a> Clone for OneOne<'a> {
//     fn clone(&self) -> OneOne<'a> {
//         println!("asdf");
//         return OneOne{_one: self._one, _oneone: self._oneone};
//     }
// }

pub fn main() {
    println!("mainmain");

    let a = 1;
    let b = a;
    println!("a is {}", a);
    println!("b is {}", b);

    let s = String::from("asdf");
    let s2 = s;
    // comment because s moved to s2
    // println!("s is {}", s);
    println!("s2 is {}", s2);

    let ss = "zxcv";
    let ss2 = ss;
    println!("ss is {}", ss);
    println!("ss2 is {}", ss2);

    let o = One{_one: 1, _two: String::from("qwerty")};
    let o2 = o;
    // comment because o moved to o2
    // println!("o is {:?}", o);
    println!("o is {:?}", o2);

    // clone struct with mutable field
    let mut o2box = Box::new(o2);
    let mut oo = OneOne{_one: 1, _oneone: &mut o2box};
    println!("oo is {:?}", oo);
    // commented, can clone object with mutable reference fields (right now can find solution)
    // let mut oo2 = oo.clone();
    // println!("oo2 is {:?}", oo2);
    let oorm = &mut oo;
    println!("oorm is {:?}", oorm);
}
