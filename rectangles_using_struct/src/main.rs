#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // functions defined in impl are called "associated functions"
    fn area(&self) -> u32 {
        // immutable borrow: &self is a short-hand for self: &Self
        // mutable borrow: &mut self is a short-hand for self: &mut Self
        // move: self is a short-hand for self: Self
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        // use :: to call functions that do not take self
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rec = Rectangle {
        width: 30,
        height: 50,
    };
    let rec1 = Rectangle {
        width: 20,
        height: 30,
    };
    println!("{:#?}", rec);
    println!("{:#?}", rec.area());
    println!("{:#?}", rec.can_hold(&rec1));
    println!("{:#?}", Rectangle::square(30));
    // rust uses automatic referencing and dereferencing
    // where rust figures out whether the method is reading (&self)
    // or mutating (&mut self) or consuming (self)
    // so rec.area() is the same as (&ric).area()
}
