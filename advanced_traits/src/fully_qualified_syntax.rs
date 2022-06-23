pub fn fully_qualified_syntax() {
    let henry = Human {};
    henry.fly();
    Pilot::fly(&henry);
    Wizard::fly(&henry);

    println!("A baby dog is called a {}", Dog::baby_name());
    // trait methods that doesn't take self needs to know for which struct it is implemented for
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}
impl Human {
    fn fly(&self) {
        println!("waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
