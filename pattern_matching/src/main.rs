fn main() {
    // or with |
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    };

    // match range with ..=
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    };
    let x = 'c';
    match x {
        'a'..='z' => println!("a to z"),
        _ => println!("something else"),
    };

    // destructuring
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    let Point { x: _x, y: _ } = p;
    match p {
        // match x: 0 and ignore remaining part
        Point { x: 0, .. } => println!("yaya"),
        _ => println!("else"),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("{} {}", first, last);
        }
    }

    // destructuring enums
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => (),
        Message::Move{x, y} => (),
        Message::Write(text) => (),
        Message::ChangeColor(r, g, b) => (),
    }

    // match guard
    let num = Some(4);
    match num  {
        Some(x) if x % 2 == 0 => println!("even {}", x),
        _ => (),
    };

    // @ operator allows match guard for a custom named variable
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        // y has custom name yaya, so it need @ operator, x does not
        Message::Move{x: 1..=10, y: yaya @ 1..=20} => println!("{} {}", x, yaya),
        _ => (),
    };
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
