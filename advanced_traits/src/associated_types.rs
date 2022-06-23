use std::ops::Add;

pub fn associated_types() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    let p = Point { x: 3, y: 3 };
    println!("{}", p.some_method(62));
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Meters(u32);
struct Millimeters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Millimeters {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

// Associated types connect a type placeholder with a trait such that the trait method definitions
// can use these placeholder types in their signatures. The implementor of a trait will specify
// the concrete type to be used in this type’s place for the particular implementation. That
// way, we can define a trait that uses some types without needing to know exactly what those
// types are until the trait is implemented.
// we can define default type parameter using <DefaultType = T> syntax
trait AssociatedType<DefaultType = i32> {
    type PlaceHolder;
    fn some_method(&self, input: DefaultType) -> Self::PlaceHolder;
}

impl AssociatedType<u32> for Point {
    type PlaceHolder = i32;

    // we can override the default type parameter here
    fn some_method(&self, input: u32) -> Self::PlaceHolder {
        input as i32 * 52
    }
}
