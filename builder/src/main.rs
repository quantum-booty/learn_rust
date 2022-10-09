#[derive(Debug)]
pub struct Mog {
    bar: String,
}

impl Mog {
    pub fn builder() -> MogBuilder {
        MogBuilder::default()
    }
}

#[derive(Default)]
pub struct MogBuilder {
    bar: String,
}

impl MogBuilder {
    pub fn new() -> MogBuilder {
        MogBuilder {
            bar: String::from("X"),
        }
    }

    pub fn name(mut self, bar: &str) -> MogBuilder {
        self.bar = bar.to_string();
        self
    }

    pub fn build(self) -> Mog {
        Mog { bar: self.bar }
    }
}

fn main() {
    let builder = Mog::builder();
    let mog = builder.name("The name is Mog, Mog wang").build();
    println!("{:?}, {:?}", mog, mog.bar);
}
