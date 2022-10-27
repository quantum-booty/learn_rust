trait Shape {
    fn draw(&self);
}
struct Circle;
struct Square;
impl Shape for Circle {
    fn draw(&self) {
        println!("I am a circle");
    }
}
impl Shape for Square {
    fn draw(&self) {
        println!("I am a Square");
    }
}

enum Shapes {
    Circle,
    Square,
}

struct ShapeFactory;

impl ShapeFactory {
    fn construct(shape: &Shapes) -> Box<dyn Shape> {
        match shape {
            Shapes::Circle => Box::new(Circle),
            Shapes::Square => Box::new(Square),
        }
    }
}

fn main() {
    let shape = ShapeFactory::construct(&Shapes::Circle);
    shape.draw();
    let shape = ShapeFactory::construct(&Shapes::Square);
    shape.draw();
}
