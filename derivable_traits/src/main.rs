use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    fmt::Display,
    hash::{Hash, Hasher},
};

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
struct X {
    field: i32,
}

impl Display for X {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mawia die die die ({})", self.field)
    }
}
fn main() {
    let x1 = X { field: 1 };
    let x2 = X { field: 2 };

    // PartialEq and Eq
    let test1 = x1.eq(&x2);
    println!("{}", test1);

    // PartialOrd and Ord
    let test2 = x1.partial_cmp(&x2);
    match test2 {
        Some(ordering) => match ordering {
            Ordering::Less => println!("less"),
            Ordering::Equal => println!("eqal"),
            Ordering::Greater => println!("greater"),
        },
        None => println!("None"),
    }

    // Debug
    // meaning we could use the {:?} formatting, its also needed for displaying unit test results
    println!("{:?}", x1);

    // Clone, Copy
    // Deriving Copy trait means when we assign x3 to x2
    // x2 is copied
    let x3 = x2;
    println!("{:?}", x3);

    // Default
    // default traits implements the default function
    let x4 = X::default();
    println!("{:?}", x4);

    // Hash
    let mut hasher = DefaultHasher::new();
    x1.hash(&mut hasher);
    println!("the hash is {}", hasher.finish());
}
