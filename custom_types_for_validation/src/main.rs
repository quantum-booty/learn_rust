mod guess {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}", value);
            }
            Guess { value }
        }

        pub fn value(&self) -> i32 {
            // getter
            self.value
        }
    }
}

fn main() {
    let g1 = guess::Guess::new(50);
    println!("{}", g1.value());
    let g2 = guess::Guess::new(101);
}
