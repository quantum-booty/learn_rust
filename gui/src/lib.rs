pub trait Draw {
    fn draw(&self);
}

// Box<dyn Draw> is a trait object
// the compiler would allow components to be any type that implement the Draw trait
// the compiler doesn't know all the types that might be used with the code that is using trait
// objects
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("drawing a button");
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("drawing a select box");
    }
}
