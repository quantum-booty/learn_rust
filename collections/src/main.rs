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

    // inserting values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // initialising from a vector using collect
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores1: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    // HashMap is needed for collect to infer which collection to transform into
    // <_, _> concrete types are not needed as the compiler can infer the types from teams and inital_scores

    // ownership
    // for types that implement the Copy trait, like i32,
    // the values are copied into the hash map
    //
    // for owned values like String, the values will be moved
    // and the hash map will be the owner of those values
    //
    // if we insert references into the hash map,
    // the values won't be moved into the hash map
    // the values that the references point to must be valid for
    // at least as long as the hash map is valid

    // accessing values
    let team_name = String::from("Blue");
    let score = scores1.get(&team_name);
    assert_eq!(score, Some(&10));

    // iterating the key and value
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // overwriting a value
    scores.insert(String::from("Nice"), 10);
    scores.insert(String::from("Nice"), 69);
    assert_eq!(scores.get("Nice"), Some(&69));

    // insert a value if key has no value
    // i.e. get or add
    scores.entry(String::from("Moggers")).or_insert(50);

    // update a value based on the old value
    let text = "hello darkness my old friend hello moggers my gf";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    for (key, val) in map {
        println!("{}: {}", key, val);
    }
}

fn main() {
    vector();
    string();
    hash_map();
}
