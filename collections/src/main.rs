use std::collections::HashMap;

fn vector() {
    // declaring new vector
    let v: Vec<i32> = Vec::new();

    // declaring vector with values using macro
    let v1 = vec![1, 2, 3, 4, 5];

    // Vec can infer type for how its used
    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    // reading via index
    // will panic if index out of range
    let third: &i32 = &v1[2];
    println!("The tird element is {}", third);

    // reading via get, which returns an Option type
    match v1.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // iterating over the values in a vector
    for i in &v1 {
        println!("{}", i);
    }

    // iterating and mutating the vector
    for i in &mut v2 {
        println!("original {}", i);
        *i += 1;
        println!("modified {}", i);
    }
}

fn string() {
    let mut s = String::new();

    // string interpolution
    // format! uses references, so does not take ownership of the parameters
    let s1 = format!("{s} yasietasitea");
    let s5 = format!("{} yasietasitea", s);
    assert_eq!(s1, s5);

    // string concatenation
    // s1 has been moved, can no longer be used after this
    // &s is a &String type, but it is coerced into &str
    let s2 = s1 + "your mom" + &s;
    // + is syntax suger for
    // fn add(self, s: &str) -> String {}

    // append to existing string
    let s3 = String::from("hsietashiteast");
    s.push_str(&s3);
    s.push('a');
    println!("{}", s);
    println!("{}", s3);

    // internal representation
    // a String is a wrapper over a Vec<u8>
    // because a character may take more than a byte
    // rust does not support string indexing as it may
    // return bytes that are invalid represent incomplete characters
    // e.g. s = "дравствуйте"
    // each character takes 2 byte
    // indexing s[0] would only get half of д

    // rust supports slicing by index
    // use with caution as it can crash the program
    let hello = "Здравствуйте";
    let s6 = &hello[0..2];
    // let &hello[0..1] will cause panic
    assert_eq!(s6, "З");

    // the best practice is to explicit if we are working with bytes or characters
    for c in hello.chars() {
        println!("{}", c);
    }
    for b in hello.bytes() {
        println!("{}", b);
    }
}

fn hash_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}

fn main() {
    // vector()
    // string()
    hash_map()
}
