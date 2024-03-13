#[derive(Debug)]
struct Rectangle {
    _width: u32,
    _height: u32,
}

impl Rectangle {
    fn _can_hold(&self, other: &Rectangle) -> bool {
        self._width > other._width && self._height > other._height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub struct Guess {
    _value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { _value: value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qwerty() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn asdf() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            _width: 8,
            _height: 7,
        };
        let smaller = Rectangle {
            _width: 5,
            _height: 1,
        };

        assert!(larger._can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            _width: 8,
            _height: 7,
        };
        let smaller = Rectangle {
            _width: 5,
            _height: 1,
        };

        assert!(!smaller._can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[should_panic(expected = "value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn qwerty_error() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
