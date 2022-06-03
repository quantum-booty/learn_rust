#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) -> isize {
        println!("{:#?}", self);
        match self {
            Message::Quit => 1,
            Message::Move { x, y } => {
                println!("{}", x * y);
                2
            }
            Message::Write(s) => {
                println!("{}", s.len());
                3
            }
            _ => 4, // catchall
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    // Option type
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    // if let, useful when only want to match one member, and do nothing for others
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("{}", max);
    }
    // this is syntax sugar for
    match config_max {
        Some(max) => println!("{}", max),
        _ => (), // ignore all other values
    }
}
